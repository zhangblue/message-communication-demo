pub mod grpc_s2c_api {
    tonic::include_proto!("api.msg.model");
}

pub const X_TASK_ID: &str = "x-task-id";
