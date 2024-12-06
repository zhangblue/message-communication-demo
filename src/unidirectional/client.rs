use message_communication_demo::grpc_s2c_api::grpc_msg_api_client::GrpcMsgApiClient;
use message_communication_demo::grpc_s2c_api::rsp::Output;
use message_communication_demo::grpc_s2c_api::{
    req, Buy, CompletionStatus, Cook, Heartbeat, Req, Wash,
};
use message_communication_demo::X_TASK_ID;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GrpcMsgApiClient::connect("http://localhost:12345").await?;

    inquiry_server(&mut client).await?;

    Ok(())
}

// 询问服务端
async fn inquiry_server(client: &mut GrpcMsgApiClient<Channel>) -> anyhow::Result<()> {
    // 第一个事件是心跳
    let mut event_msg = Req {
        input: Some(req::Input::HeartBeat(Heartbeat {
            count: 1,
            agent_id: "agent_1".to_string(),
        })),
    };
    // 第一个任务的task_id是空的
    let mut task_id_opt: Option<String> = None;

    let mut do_next = true;
    while do_next {
        // 创建新请求
        let mut request = Request::new(event_msg);
        if let Some(task_id) = task_id_opt {
            let metadata_value = MetadataValue::try_from(task_id)?;
            request.metadata_mut().insert(X_TASK_ID, metadata_value);
        }

        // 发送请求，并得到服务端的回复
        let response = client.unidirectional(request).await?;
        // 得到当前任务id，任务id从源数据的 x-task-id 字段中获取
        let task_id = response
            .metadata()
            .get(X_TASK_ID)
            .and_then(|value| value.to_str().ok())
            .unwrap();
        // 重制task_id
        task_id_opt = Some(task_id.to_string());

        if let Some(output) = &response.get_ref().output {
            match output {
                Output::DoNothing(_) => {
                    // 此类型说明服务端没有事件让客户端执行
                    do_next = false;
                    event_msg = Req { input: None };
                }
                Output::Buy(event) => {
                    // 让客户端执行买菜逻辑
                    let status = do_buy_event(event, task_id).await;
                    event_msg = Req {
                        input: Some(req::Input::CompletionStatus(status)),
                    };
                }
                Output::Wash(event) => {
                    // 让客户端执行洗菜逻辑
                    let status = do_wash_event(event, task_id).await;
                    event_msg = Req {
                        input: Some(req::Input::CompletionStatus(status)),
                    };
                }
                Output::Cook(event) => {
                    // 让客户端执行做饭逻辑
                    let status = do_cook_event(event, task_id).await;
                    event_msg = Req {
                        input: Some(req::Input::CompletionStatus(status)),
                    };
                }
            }
        } else {
            // 服务端回复错误。则重制所有参数
            event_msg = Req { input: None };
            do_next = false;
            task_id_opt = None;
        }
    }

    Ok(())
}

async fn do_buy_event(event: &Buy, task_id: &str) -> CompletionStatus {
    let msg = event.msg.as_str();
    println!("开始买菜 [{}], task_id = [{}]", msg, task_id);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("买菜结束 [{}] 用时5秒", task_id);
    CompletionStatus {
        code: 1,
        msg: String::from("买菜完成"),
    }
}

async fn do_wash_event(event: &Wash, task_id: &str) -> CompletionStatus {
    let msg = event.msg.as_str();
    println!("开始洗菜 [{}], task_id = [{}]", msg, task_id);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("洗菜结束 [{}] 用时5秒", task_id);
    CompletionStatus {
        code: 1,
        msg: String::from("洗菜完成"),
    }
}

async fn do_cook_event(event: &Cook, task_id: &str) -> CompletionStatus {
    let msg = event.msg.as_str();
    println!("开始做饭 [{}], task_id = [{}]", msg, task_id);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    println!("做饭结束 [{}] 用时5秒", task_id);
    CompletionStatus {
        code: 1,
        msg: String::from("洗菜完成"),
    }
}
