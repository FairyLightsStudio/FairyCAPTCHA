本文件假定你已经看完了 [ README ](./README.md)。

安装分两步：

1. 在 APP前端 集成 PoWCAPTCHA Web Component
2. 在 APP后端 集成 PoWCAPTCHA后端


### 在前端集成 PoWCAPTCHA Web Component

#### 

#### 

### 在后端集成 PoWCAPTCHA

#### 1. 运行 PoWCAPTCHA后端

你可以自己部署本项目的后端。

后端使用环境变量配置。

```
# 必填项
POWCAPTCHA_SERVICE_DATABASE_URL="postgres://sqlx:sqls0302@localhost"

# 选填，默认为 "[::]:8080"
POWCAPTCHA_SERVICE_ADDRESS="[::]:8080"
```

#### 通过 gRPC 调用 PoWCAPTCHA后端

