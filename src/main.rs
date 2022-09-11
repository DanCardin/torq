use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use futures_util::stream::TryStreamExt;
use std::{thread, time};

async fn version(docker: Docker) {
    let options = Some(CreateImageOptions {
        from_image: "postgres:12",
        ..Default::default()
    });

    docker
        .create_image(options, None, None)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let container_options = Some(CreateContainerOptions { name: "woah" });
    let config = Config {
        image: Some("postgres:12"),
        env: Some(vec!["POSTGRES_PASSWORD=password"]),
        ..Default::default()
    };
    let container = docker
        .create_container(container_options, config)
        .await
        .unwrap();
    println!("{}", container.id);

    docker
        .start_container("woah", None::<StartContainerOptions<String>>)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let result = version(docker).await;
    println!("{result:?}");
}
