# PoWCAPTCHA

PoWCAPTCHA 是一款 使用 PoW 方案，通过要求用户设备支付一些它的算力，起到防御 特别暴力的DoS攻击 的效果的开源、开放的验证码模块。

PoWCAPTCHA 受到 [Anubis](https://github.com/TecharoHQ/anubis) 的 [启发](http://anubis.techaro.lol/docs/design/why-proof-of-work/)。

区别是，Anubis 是部署在web app前的、开箱即用的防火墙，本项目需要你改动已有项目，主动调用 PoWCAPTCHA 方可发挥作用。

### 原理

> **工作量证明（Proof-of-Work，PoW）** 是一种对应服务与资源滥用、或是拒绝服务攻击的经济对策。一般要求用户进行一些耗时适当的复杂运算，并且答案能被服务方快速验算，以此耗用的时间、设备与能源做为担保成本，以确保服务与资源是被真正的需求所使用。    

> 工作量证明系统的核心特性在于其非对称性：证明者需 **完成适度困难（但可行）** 的计算工作，而验证者能**极高效地**验证结果。——— [维基百科](https://zh.wikipedia.org/wiki/%E5%B7%A5%E4%BD%9C%E9%87%8F%E8%AD%89%E6%98%8E)

你可以使用本项目：

- 保护网站的登录服务。让DoS攻击者几乎不可能暴力破解某个用户的密码。
- 保护UGC网站的评论服务。使DoS攻击者制造垃圾内容的速度不至于那么离谱。
- ...


## 快速上手

```mermaid
sequenceDiagram
  actor user as 用户
  actor fe as APP前端
  actor fe_pow as PoWCAPTCHA Web Component
  actor be as APP后端
  actor PoWCAPTCHA as PoWCAPTCHA后端

  autonumber
  user ->> fe: 打开登录页
  fe ->> fe_pow: 识别到登录api被保护<br>传入 Access Key ID、选填当前 action（login）<br>加载 PoWCAPTCHA 小组件
  fe_pow ->> fe: 组件文案：🛡️此操作由 PoWCAPTCHA 保护，以防滥用
  Note left of fe_pow: 组件预先开始解决试卷<br>尽量避免用户主动为 PoWCAPTCHA 等待
  fe_pow ->> PoWCAPTCHA: 带着 PoWservice Access Key ID、当前 action（如果有） 请求验证
  Note right of PoWCAPTCHA: 确认 当前请求网域<br/>在Access Key ID 允许的域名列表内
  user ->> fe: 用户开始输入用户名密码
  PoWCAPTCHA ->> PoWCAPTCHA: 生成试卷
  PoWCAPTCHA ->> fe_pow: 下发考试session、试卷
  Note right of PoWCAPTCHA: 下发考试session指的是<br/>下发session的Access Key ID、Secret
  fe_pow ->> fe_pow: 消耗用户的算力来答题
  Note left of fe_pow: PoWCAPTCHA向设备收取算力<br/>不要求用户参加考试
  fe_pow ->> PoWCAPTCHA: 提交考试session、答卷
  PoWCAPTCHA ->> PoWCAPTCHA: 检查答卷、生成证明设备完成了一份试卷的token
  PoWCAPTCHA ->> fe_pow: 五分钟后失效的临时 token
  fe_pow ->> fe_pow: 把 token 放到缓冲池内
  user ->> fe: 点击“登录”
  fe -> fe_pow: 前端向 PoWCAPTCHA 组件 请求 token
  alt 缓冲池 内没有有效token
    Note left of fe_pow: token 离失效期小于10秒就判其无效<br/>（给前端 -> APP后端 -> PoWCAPTCHA后端 链路传递token的时间）
    fe_pow ->> fe: 组件文案：⏳正在使用设备算力通过验证
    fe_pow -> fe: 发起一次做题流程 获取一个有效 token
    fe_pow ->> fe: 组件文案：✅已通过验证
    fe_pow ->> fe: 返回有效 token
  end
  alt 缓冲池 内有有效token
    fe_pow ->> fe: 从缓冲池拿个有效 token 返回
  end
  fe ->> be: 前端附带token调用被保护的api
  be ->> PoWCAPTCHA: 询问可否放行这个token对应的请求
  alt token 合法
    PoWCAPTCHA ->> PoWCAPTCHA: 吊销该 token
    PoWCAPTCHA ->> be: 嗯呐，没问题，可以放行
    be ->> be: 执行业务逻辑
    be ->> fe: api结果
  end
  alt token 不合法
    PoWCAPTCHA ->> be: 有问题，不可以放行
    be ->> fe: 403
  end
```

> 为方便大家理解，把 Challenge 比作了试卷、Solution 比作了答卷

以下提供两种方法让你的项目集成 PoWCAPTCHA，选择最合适的那个。

### Rust tokio 后端：直接使用 pow-captcha Crate

若你就是用 Rust tokio / 基于tokio的框架 写的后端，那你可以直接把 PoWCAPTCHA后端 集成到你的网站后端里面去。



引用 [Crates/pow-captcha](https://crates.io/crates/pow-captcha) 到你的项目。

pow-captcha Crate 提供以下函数：

- 生成试卷
- 检查答卷是否正确
- 


### 其他后端：通过 RPC 使用 PoWCAPTCHA
你也可以单独运行本项目，后端和


## RoadMap

- 探索使用 RandomX 算法 代替 SHA加密哈希函数 以取得更强抗ASIC特性的可能性

