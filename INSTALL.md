本文件假定你已经看完了 [ 👀架构一览 ](./README.md)。


安装分两步：

1. 在 APP前端 集成 PoWCAPTCHA Web Component
2. 在 APP后端 集成 PoWCAPTCHA后端


### 在前端集成 PoWCAPTCHA Web Component



### 在后端集成 PoWCAPTCHA


以下提供两种方法让你的项目后端集成 PoWCAPTCHA，选择最合适的那个。

#### Rust tokio 后端：直接使用 pow-captcha Crate

若你就是用 Rust tokio / 基于tokio的框架 写的后端，那你可以直接把 PoWCAPTCHA后端 集成到你的网站后端里面去。



引用 [Crates/pow-captcha](https://crates.io/crates/pow-captcha) 到你的项目。

pow-captcha Crate 提供以下函数：

- 生成试卷
- 检查答卷是否正确
- 


#### 其他后端：通过 RPC 使用 PoWCAPTCHA
你也可以单独运行本项目，后端和

