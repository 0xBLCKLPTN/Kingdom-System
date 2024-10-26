/*                          MIT License

Copyright (c) 2024 Daniil Ermolaev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

pub use tonic::{ transport::Server, Request, Response, Status };

pub mod grpc_proto {
    tonic::include_proto!("instance");
}

pub use grpc_proto:: {
    CreateInstanceRequest  ,CreateInstanceResponse,
    GetInstanceRequest     ,GetInstanceResponse,
    GetAllInstancesRequest  ,GetAllInstancesResponse,
    SignUpRequest          ,SignUpResponse,
    Token,
};

pub use grpc_proto::instance_service_server::InstanceServiceServer;
pub use grpc_proto::instance_service_server::InstanceService;


use crate::json_engine::*;
use crate::instance::InstanceManager;

use std::sync::{ Arc, Mutex };


#[derive(Debug, Default)]
pub struct GRPCInstanceManager {
    pub instance_manager: Arc<Mutex<InstanceManager>>,
}

#[tonic::async_trait]
impl InstanceService for GRPCInstanceManager
{
    async fn create_instance(
        &self, request: Request<CreateInstanceRequest>,
    ) -> Result<Response<CreateInstanceResponse>, Status> {
        let inner = request.into_inner();
        let engine_type = inner.engine_type;
        let name = inner.name;
        let mut im = self.instance_manager.lock().unwrap();
        let id = im.create_instance(&engine_type, &name).unwrap();

        Ok(
            Response::new( CreateInstanceResponse { instance: id } )
        )
    }

    async fn get_instance(
        &self, request: Request<GetInstanceRequest>,
    ) -> Result<Response<GetInstanceResponse>, Status> {

        let instance_name = request.into_inner().instance_name;
        let im = self.instance_manager.lock().unwrap();

        if let Some(instance) = im.get_instance(&instance_name) {
            return Ok( Response::new(GetInstanceResponse { instance: instance.name.clone(), }));
        }
        Err( Status::not_found("Instance not found") )
    }

    async fn get_all_instances(
        &self, request: Request<GetAllInstancesRequest>,
    ) -> Result<Response<GetAllInstancesResponse>, Status> {

        let im = self.instance_manager.lock().unwrap();
        let mut re_instances: Vec<grpc_proto::Instance> = vec![];

        for instance in &im.instances {
            re_instances.push(
                grpc_proto::Instance {
                    name: instance.name.clone(),
                    engine: "not implemented".into(),
                }
            )
        }
        let response = GetAllInstancesResponse { instances: re_instances };
        Ok( Response::new( response ))
    }

    async fn sign_up(
        &self, request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {

        let inner = request.into_inner();
        let mut im = self.instance_manager.lock().unwrap();

        let mut key: String = String::new();

        if !im.authenticated_apps.contains_key(&inner.app_name) {
            key = im.sign_up(inner.app_name);
        } else {
            key = "whoops...".to_string();
        }
        let response = SignUpResponse { secret_key: key };
        im.get_all_apps();
        Ok( Response::new(response) )
    }
}

