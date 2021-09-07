use std::{collections::HashMap, sync::Arc};

use crate::{NextMiddleware, NextMiddlewareGroup};

mod path;

use path::Path;
use poem::{
    http::Method, route::Route, Endpoint, EndpointExt, IntoEndpoint, Response, RouteMethod,
};

///poem中路由树，类似于组
pub struct Chapter {
    chapters: Vec<Chapter>,
    middleware: Vec<Arc<dyn NextMiddleware>>,
    path: Path,
    method_endpoint: HashMap<Method, Box<dyn Endpoint<Output = Response>>>,
}

impl Chapter {
    ///创建子路由
    pub fn at<F>(mut self, path: &str, create_chapter: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.chapters.push(create_chapter(Chapter {
            chapters: vec![],
            middleware: self.middleware.clone(),
            path: self.path.clone().append(path),
            method_endpoint: HashMap::new(),
        }));

        self
    }

    //添加一个子路由
    pub fn add(mut self, chapter: Chapter) -> Self {
        let Chapter {
            mut chapters,
            mut middleware,
            path,
            method_endpoint,
        } = chapter;

        fn add_path(
            mut chapter: Chapter,
            path: Path,
            mut pre_middleware: Vec<Arc<dyn NextMiddleware>>,
        ) -> Chapter {
            chapter.path = path.clone().append(&chapter.path.to_string());

            let mut new_middleware = chapter.middleware.clone();

            new_middleware.append(&mut pre_middleware);
            chapter.middleware = new_middleware;

            chapter.chapters = chapter
                .chapters
                .into_iter()
                .map(|chapter| add_path(chapter, path.clone(), pre_middleware.clone()))
                .collect();

            chapter
        }

        let root_path = self.path.clone();
        let root_middleware = self.middleware.clone();
        chapters = chapters
            .into_iter()
            .map(|chapter| add_path(chapter, root_path.clone(), root_middleware.clone()))
            .collect();

        let mut new_middleware = self.middleware.clone();

        new_middleware.append(&mut middleware);

        self.chapters.push(Chapter {
            path: self.path.clone().append(&path.to_string()),
            chapters,
            middleware: new_middleware,
            method_endpoint,
        });

        self
    }

    //添加一个服务
    pub fn method(mut self, method: Method, ep: impl IntoEndpoint) -> Self {
        self.method_endpoint
            .insert(method, Box::new(ep.into_endpoint().map_to_response()));
        self
    }

    #[doc(hidden)]
    fn build(self) -> Vec<RouteDescriptor> {
        let Chapter {
            chapters,
            middleware,
            path,
            method_endpoint,
        } = self;

        let method_route =
            method_endpoint
                .into_iter()
                .fold(RouteMethod::new(), |method_route, (method, ep)| {
                    let method_route = method_route.method(
                        method,
                        ep.with(NextMiddlewareGroup {
                            next_middleware: middleware.clone(),
                        }),
                    );
                    method_route
                });
        let local_endpoints = vec![RouteDescriptor {
            path: path,
            route_method: method_route,
        }];

        let sub_endpoints = chapters.into_iter().flat_map(Chapter::build);

        local_endpoints.into_iter().chain(sub_endpoints).collect()
    }
}

pub struct RouteDescriptor {
    path: Path,
    route_method: RouteMethod,
}

pub trait RouterBuilder {
    fn add(self, chapter: Chapter) -> Self;
}

impl RouterBuilder for Route {
    fn add(mut self, chapter: Chapter) -> Self {
        for RouteDescriptor { path, route_method } in chapter.build() {
            self = self.at(&path.to_string(), route_method)
        }

        self
    }
}
