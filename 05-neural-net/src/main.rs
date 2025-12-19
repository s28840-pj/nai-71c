#![doc = include_str!("../README.md")]

use std::marker::PhantomData;

use burn::{
    Tensor,
    backend::{Autodiff, Wgpu, wgpu::WgpuDevice},
    config::Config,
    data::{
        dataloader::{DataLoaderBuilder, batcher::Batcher},
        dataset::Dataset,
    },
    module::Module,
    nn::{Linear, LinearConfig, Relu},
    optim::AdamConfig,
    prelude::Backend,
    tensor::{Int, backend::AutodiffBackend},
    train::{ClassificationOutput, LearnerBuilder, TrainOutput, TrainStep, ValidStep},
};
use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};

// data
fn quality_as_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let quality = u8::deserialize(d)?;
    Ok(quality > 6)
}

#[derive(Debug, Clone, Deserialize)]
pub struct WineRecord {
    #[serde(rename = "fixed acidity")]
    pub fixed_acidity: f32,
    #[serde(rename = "volatile acidity")]
    pub volatile_acidity: f32,
    #[serde(rename = "citric acid")]
    pub citric_acid: f32,
    #[serde(rename = "residual sugar")]
    pub residual_sugar: f32,
    pub chlorides: f32,
    #[serde(rename = "free sulfur dioxide")]
    pub free_sulfur_dioxide: f32,
    #[serde(rename = "total sulfur dioxide")]
    pub total_sulfur_dioxide: f32,
    pub density: f32,
    #[serde(rename = "pH")]
    pub ph: f32,
    pub sulphates: f32,
    pub alcohol: f32,
    #[serde(deserialize_with = "quality_as_bool")]
    pub quality: bool,
}

#[derive(Clone)]
pub struct WineDataset {
    records: Vec<WineRecord>,
}

impl Dataset<WineRecord> for WineDataset {
    fn get(&self, index: usize) -> Option<WineRecord> {
        self.records.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.records.len()
    }
}

impl WineDataset {
    fn split(self, ratio: f64) -> (WineDataset, WineDataset) {
        assert!(ratio >= 0. && ratio <= 1.);
        let split_idx = (self.len() as f64 * ratio) as usize;
        let (a, b) = self.records.split_at(split_idx);
        let a = WineDataset {
            records: a.to_vec(),
        };
        let b = WineDataset {
            records: b.to_vec(),
        };
        (a, b)
    }
}

fn winequality() -> WineDataset {
    const DATASET_URL: &str = "https://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-red.csv";

    let resp = reqwest::blocking::get(DATASET_URL).unwrap().text().unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(resp.as_bytes());
    let records: Vec<WineRecord> = rdr
        .deserialize()
        .map(|r| r.expect("CSV mapping error"))
        .collect();
    WineDataset { records }
}

// model

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    l1: Linear<B>,
    l2: Linear<B>,
    l3: Linear<B>,
    activation: Relu,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    #[config(default = 11)]
    input_size: usize,
    #[config(default = 64)]
    hidden_size: usize,
    #[config(default = 11)]
    num_classes: usize, // Quality scores are 0-10
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            l1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            l2: LinearConfig::new(self.hidden_size, self.hidden_size).init(device),
            l3: LinearConfig::new(self.hidden_size, self.num_classes).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.activation.forward(self.l1.forward(input));
        let x = self.activation.forward(self.l2.forward(x));
        self.l3.forward(x)
    }
}

// training

#[derive(Clone, Debug)]
pub struct WineBatch<B: Backend> {
    pub inputs: Tensor<B, 2>,
    pub targets: Tensor<B, 1, Int>,
}

#[derive(Clone)]
pub struct WineBatcher<B: Backend> {
    _backend: PhantomData<B>,
}

impl<B: Backend> WineBatcher<B> {
    pub fn new() -> Self {
        WineBatcher {
            _backend: PhantomData,
        }
    }
}

impl<B: Backend> Batcher<B, WineRecord, WineBatch<B>> for WineBatcher<B> {
    fn batch(&self, items: Vec<WineRecord>, device: &B::Device) -> WineBatch<B> {
        let inputs = items
            .iter()
            .map(|item| {
                Tensor::<B, 1>::from_floats(
                    [
                        item.fixed_acidity,
                        item.volatile_acidity,
                        item.citric_acid,
                        item.residual_sugar,
                        item.chlorides,
                        item.free_sulfur_dioxide,
                        item.total_sulfur_dioxide,
                        item.density,
                        item.ph,
                        item.sulphates,
                        item.alcohol,
                    ],
                    device,
                )
            })
            .collect();

        let targets: Vec<_> = items.iter().map(|item| item.quality as i32).collect();
        let targets = Tensor::from_ints(targets.as_slice(), device);

        WineBatch {
            inputs: Tensor::stack(inputs, 0),
            targets,
        }
    }
}

impl<B: Backend> Model<B> {
    pub fn forward_classification(
        &self,
        inputs: Tensor<B, 2>,
        targets: Tensor<B, 1, Int>,
    ) -> ClassificationOutput<B> {
        let output = self.forward(inputs);
        let loss = burn::nn::loss::CrossEntropyLossConfig::new()
            .init(&output.device())
            .forward(output.clone(), targets.clone());

        ClassificationOutput::new(loss, output, targets)
    }
}

impl<B: AutodiffBackend> TrainStep<WineBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: WineBatch<B>) -> TrainOutput<ClassificationOutput<B>> {
        let item = self.forward_classification(batch.inputs, batch.targets);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<WineBatch<B>, ClassificationOutput<B>> for Model<B> {
    fn step(&self, batch: WineBatch<B>) -> ClassificationOutput<B> {
        self.forward_classification(batch.inputs, batch.targets)
    }
}

type MyBackend = Autodiff<Wgpu>;

// fn train<B: AutodiffBackend>(device: &B::Device) {

// }

fn main() {
    let device = WgpuDevice::default();

    let dataset = winequality();
    let (training_data, test_data) = dataset.clone().split(0.8);

    println!("Starting training on {} samples...", training_data.len());
    let config = ModelConfig::new();
    let batcher_train = WineBatcher::<MyBackend>::new();
    let dataloader_train = burn::data::dataloader::DataLoaderBuilder::new(batcher_train)
        .batch_size(32)
        .shuffle(42)
        .build(training_data);

    let batcher_valid = WineBatcher::<<MyBackend as AutodiffBackend>::InnerBackend>::new();
    let dataloader_valid = DataLoaderBuilder::new(batcher_valid)
        .batch_size(32)
        .shuffle(42)
        .build(test_data);

    let artifact_dir = "/tmp/burn-wine-model";
    let learner = LearnerBuilder::new(artifact_dir)
        .metric_train_numeric(burn::train::metric::AccuracyMetric::new())
        .with_file_checkpointer(burn::record::CompactRecorder::new())
        .num_epochs(15)
        .build(
            config.init::<MyBackend>(&device),
            AdamConfig::new().init(),
            1e-3,
        );

    let trained_model = learner.fit(dataloader_train, dataloader_valid).model;

    let batcher = WineBatcher::<<MyBackend as AutodiffBackend>::InnerBackend>::new();
    let dataloader = DataLoaderBuilder::new(batcher).batch_size(1).build(dataset);

    let total = dataloader.num_items();
    let correct = dataloader
        .iter()
        .map(|batch| {
            let output = trained_model.forward(batch.inputs);
            let predicted = output.argmax(1).reshape([-1]);

            predicted
                .equal(batch.targets)
                .into_data()
                .convert::<i32>()
                .iter()
                .next()
                .unwrap()
        })
        .filter(|&correct: &i32| correct == 1)
        .count();

    let accuracy = (correct as f64 / total as f64) * 100.;
    println!("Final accuracy on full dataset: {accuracy:.2}%");
}
