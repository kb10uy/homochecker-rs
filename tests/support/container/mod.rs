mod repository;
mod service;

use self::service::{MockAvatarService, MockHomoRequestService};
use homochecker_rs::{repository::Repositories, service::Services, Container};

#[derive(Default, Clone)]
pub struct MockContainer {
    pub repositories: MockRepositories,
    pub services: MockServices,
}

#[derive(Default, Clone)]
pub struct MockRepositories {
    pub user: MockUserRepository,
    pub avatar: MockAvatarRepository,
}

#[derive(Default, Clone)]
pub struct MockServices {
    pub avatar: MockAvatarService,
    pub homo_request: MockHomoRequestService,
}

impl Container for MockContainer {
    type Repositories = MockRepositories;
    type Services = MockServices;

    fn repositories(&self) -> MockRepositories {
        self.repositories.clone()
    }

    fn services(&self) -> MockServices {
        self.services.clone()
    }
}

impl Repositories for MockRepositories {
    type User = MockUserRepository;
    type Avatar = MockAvatarRepository;

    fn user(&self) -> MockUserRepository {
        self.user.clone()
    }

    fn avatar(&self) -> MockAvatarRepository {
        self.avatar.clone()
    }
}

impl Services for MockServices {
    type Avatar = MockAvatarService;
    type HomoRequest = MockHomoRequestService;

    fn avatar(&self) -> MockAvatarService {
        self.avatar.clone()
    }

    fn homo_request(&self) -> MockHomoRequestService {
        self.homo_request.clone()
    }
}
