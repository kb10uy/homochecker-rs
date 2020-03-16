//! Contains repository adapters.

mod repository;
mod service;

use self::{
    repository::{AvatarRepository, UserRepository},
    service::{AvatarService, HomoRequestService},
};
use homochecker_rs::{
    repository::Repositories as RepositoriesInterface, service::Services as ServicesInterface,
    Container as ContainerInterface,
};
use std::{sync::Arc, time::Duration};

use redis::aio::Connection as RedisConnection;
use reqwest::{redirect::Policy as RedirectPolicy, Client as ReqwestClient};
use tokio::sync::Mutex;
use tokio_postgres::Client as PostgresClient;

#[derive(Clone)]
pub struct Container {
    repositories: Repositories,
    services: Services,
}

impl Container {
    pub fn new(repositories: Repositories, services: Services) -> Container {
        Container {
            repositories,
            services,
        }
    }
}

impl ContainerInterface for Container {
    type Repositories = Repositories;
    type Services = Services;

    fn repositories(&self) -> Repositories {
        self.repositories.clone()
    }

    fn services(&self) -> Services {
        self.services.clone()
    }
}

#[derive(Clone)]
pub struct Repositories {
    postgres: Arc<PostgresClient>,
    redis: Arc<Mutex<RedisConnection>>,
}

impl Repositories {
    pub fn new(postgres: PostgresClient, redis: RedisConnection) -> Repositories {
        Repositories {
            postgres: Arc::new(postgres),
            redis: Arc::new(Mutex::new(redis)),
        }
    }
}

impl RepositoriesInterface for Repositories {
    type User = UserRepository;
    type Avatar = AvatarRepository;

    fn user(&self) -> UserRepository {
        UserRepository::new(self.postgres.clone())
    }

    fn avatar(&self) -> AvatarRepository {
        AvatarRepository::new(self.redis.clone())
    }
}

#[derive(Clone)]
pub struct Services {
    avatar_client: Arc<ReqwestClient>,
    homo_client: Arc<ReqwestClient>,
}

impl Services {
    pub fn new() -> Services {
        let avatar_client = Arc::new(ReqwestClient::new());
        let homo_client = Arc::new(
            ReqwestClient::builder()
                .redirect(RedirectPolicy::custom(|attempt| {
                    // HTTP -> HTTPS のリダイレクトだけ追う
                    let prev = &attempt.previous()[0];
                    let next = attempt.url();
                    if prev.domain() == next.domain() {
                        attempt.follow()
                    } else {
                        attempt.stop()
                    }
                }))
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        );

        Services {
            avatar_client,
            homo_client,
        }
    }
}

impl ServicesInterface for Services {
    type Avatar = AvatarService;
    type HomoRequest = HomoRequestService;

    fn avatar(&self) -> AvatarService {
        AvatarService::new(self.avatar_client.clone())
    }

    fn homo_request(&self) -> HomoRequestService {
        HomoRequestService::new(self.homo_client.clone())
    }
}
