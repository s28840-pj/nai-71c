#![doc = include_str!("../README.md")]

use linfa::{
    Dataset,
    metrics::ToConfusionMatrix,
    traits::{Fit, Predict},
};
use linfa_datasets::array_from_csv;
use linfa_svm::Svm;
use linfa_trees::DecisionTree;
use ndarray::{Ix1, s};

fn run_models(name: &str, dataset: Dataset<f64, bool, Ix1>, (yes, no): (&str, &str)) {
    let tag_targets = |&target: &bool| if target { yes } else { no };

    println!("| {}", name);
    let testing = dataset.clone();
    let testing = testing.map_targets(tag_targets);

    println!("Drzewo decyzyjne:");
    let tree = DecisionTree::params().fit(&dataset).unwrap();

    let cm = tree
        .predict(&testing)
        .map(tag_targets)
        .confusion_matrix(&testing)
        .unwrap();

    println!("{cm:?}");
    println!("dokładność {}, MCC {}", cm.accuracy(), cm.mcc());

    println!();

    println!("SVM (Kernel gaussowski):");
    let svm = Svm::<_, bool>::params()
        .pos_neg_weights(1_000., 100.)
        .gaussian_kernel(60.)
        .fit(&dataset)
        .unwrap();

    let cm = svm
        .predict(&testing)
        .map(tag_targets)
        .confusion_matrix(&testing)
        .unwrap();

    println!("{cm:?}");
    println!("dokładność {}, MCC {}", cm.accuracy(), cm.mcc());

    println!();

    println!("SVM (Kernel wielomianowy):");
    let svm = Svm::<_, bool>::params()
        .pos_neg_weights(10., 10.)
        .polynomial_kernel(2., 1.2)
        .fit(&dataset)
        .unwrap();

    let cm = svm
        .predict(&testing)
        .map(tag_targets)
        .confusion_matrix(&testing)
        .unwrap();

    println!("{cm:?}");
    println!("dokładność {}, MCC {}", cm.accuracy(), cm.mcc());
    println!("--------------------------------------------");
}

fn main() {
    let heart_disease = {
        let data = array_from_csv(
            include_bytes!("../heart-disease.csv").as_slice(),
            true,
            b',',
        )
        .unwrap();

        let (data, targets) = (
            data.slice(s![.., 0..13]).to_owned(),
            data.column(13).to_owned(),
        );

        let feature_names = vec![
            "age", "sex", "cp", "trestbps", "chol", "fbs", "restecg", "thalach", "exang",
            "oldpeak", "slope", "ca", "thal",
        ];

        Dataset::new(data, targets)
            .map_targets(|x| *x == 1.)
            .with_feature_names(feature_names)
    };
    let winequality = linfa_datasets::winequality().map_targets(|x| *x > 6);

    run_models("Heart Disease", heart_disease, ("chory", "zdrowy"));
    run_models("Wine Quality", winequality, ("dobre", "niedobre"));
}
