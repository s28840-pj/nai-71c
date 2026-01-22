mod args;
mod config;
mod types;
use args::Args;
use border_core::{
    Agent, Configurable, Env as _, Evaluator as _, ReplayBufferBase, StepProcessor, Trainer,
    generic_replay_buffer::SimpleStepProcessorConfig, record::Recorder,
};
use border_mlflow_tracking::MlflowTrackingClient;
use border_tensorboard::TensorboardRecorder;
use config::DqnAtariConfig;
use types::*;

const MODEL_DIR: &str = "./model";
const MLFLOW_EXPERIMENT_NAME: &str = "Atari";
const MLFLOW_TAGS: &[(&str, &str)] = &[("algo", "dqn"), ("backend", "tch")];

fn create_agent(config: &DqnAtariConfig) -> Box<dyn Agent<Env, ReplayBuffer>> {
    let n_actions = Env::build(&config.env_config, 0)
        .unwrap()
        .get_num_actions_atari() as i64;
    let agent_config = config.agent_config.clone().out_dim(n_actions);
    Box::new(Dqn::build(agent_config))
}

fn create_recorder(
    args: &Args,
    config: Option<&DqnAtariConfig>,
) -> Box<dyn Recorder<Env, ReplayBuffer>> {
    // if let Some(mlflow_run_name) = &args.mlflow_run_name {
    //     let client = MlflowTrackingClient::new("http://localhost:8080")
    //         .set_experiment(MLFLOW_EXPERIMENT_NAME).unwrap();
    //     let recorder_run = client.create_recorder(format!("{}_tch", mlflow_run_name)).unwrap();
    //     if let Some(config) = config {
    //         recorder_run.log_params(&config).unwrap();
    //         recorder_run.set_tag("env", &args.name).unwrap();
    //         recorder_run.set_tags(MLFLOW_TAGS).unwrap();
    //     }
    //     Ok(Box::new(recorder_run))
    // } else {
    let model_dir = format!("{}/{}", MODEL_DIR, &args.name);
    Box::new(TensorboardRecorder::new(&model_dir, &model_dir, false))
    // }
}

fn train(config: &DqnAtariConfig) -> () {
    let env_config_train = config.clone_env_config();
    let env_config_eval = config.clone_env_config().eval();
    let step_proc_config = SimpleStepProcessorConfig {};

    let mut trainer = Trainer::build(config.clone_trainer_config());
    let env = Env::build(&env_config_train, 0).unwrap();
    let step_proc = StepProc::build(&step_proc_config);
    let mut agent = create_agent(config);
    let mut buffer = ReplayBuffer::build(&config.clone_replay_buffer_config());
    let mut recorder = create_recorder(&config.args, Some(config));
    let mut evaluator = Evaluator::new(&env_config_eval, 0, 1).unwrap();

    trainer
        .train(
            env,
            step_proc,
            &mut agent,
            &mut buffer,
            &mut recorder,
            &mut evaluator,
        )
        .unwrap();
}

fn eval(config: &DqnAtariConfig) -> () {
    let env_config = config.clone_env_config();
    let mut agent = create_agent(config);
    let mut evaluator = Evaluator::new(&env_config, 0, 5).unwrap();

    // recorder is used to load model parameters
    let recorder = create_recorder(&config.args, None);
    recorder.load_model("best".as_ref(), &mut agent).unwrap();

    let _ = evaluator.evaluate(&mut agent);
}

fn main() {
    let config: DqnAtariConfig = Args::mock().into();

    match config.args.mode.as_str() {
        "train" => train(&config),
        "eval" => eval(&config),
        _ => panic!("mode must be either 'train' or 'eval'"),
    }
}
