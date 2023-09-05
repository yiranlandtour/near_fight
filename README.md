fight
==================

This app was initialized with [create-near-app]


Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/index.html` is a great
   place to start exploring. Note that it loads in `/src/index.js`, where you
   can learn how the frontend connects to the NEAR blockchain.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn
   run test`.


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `fight.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `fight.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account fight.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'fight.YOUR-NAME.testnet'


Step 3: deploy!
---------------

One command:

    yarn deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages



===================================================================================================================


# 去中心化募捐平台
## 背景
针对社会上急需帮助的群体性事件，如：涿州水灾，流浪动物救助，留守儿童失学 等问题，又因为xxx等募捐机构不公开账目，存在侵吞募捐款的可能，现基于区块链去中心化的能力，真正做到公开透明的原则搭建此去中心化募捐平台，发布募捐活动，募集NEAR，为救助对象提供帮助；
## 平台主要能力
```shell
1、发布募捐活动，募集near币
2、用户参与募捐活动，向募捐指定账户转入NEAR
3、展示募捐活动详情：参与募捐的人和当前募捐到的NEAR
```

## 代码仓库
合约代码：https://github.com/guozhouwei/near_crowdfunding_site/blob/main/near_crowdfunding/src/lib.rs  
near-api-js: https://github.com/guozhouwei/near_crowdfunding_site/blob/main/src/main.ts

## 合约
### 业务逻辑图
![avatar](https://github.com/guozhouwei/near_crowdfunding/blob/master/images/%E5%86%B3%E7%AD%96%E6%B5%81%E7%A8%8B%E5%9B%BE.png)

### 部署和交互
#### 账户
假设你注册了4个测试网账户:
```shell
1. 主账户（同合约部署签名账户）owner123.testnet
2. 合约账户 contract1234501.testnet
3. 募捐人账户 zhouzhou_near.testnet
```

##### 以上账户私钥保存在 legacy keychain 中
```shell
near account import-account using-seed-phrase "${YOUR_SEED_PHRASE}" --seed-phrase-hd-path 'm/44'\''/397'\''/0'\''' network-config testnet

```

#### 编译募捐合约
1. 进入项目目录
    ```shell
   cd .
    ```
2. 安装 WASM 工具链
    ```shell
   rustup target add wasm32-unknown-unknown
    ```
3. 编译合约
    ```shell
   RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release
    ```
4. 将合约 WASM 文件移动到项目根目录下方便后续操作
    ```shell
   mkdir -p ./res && cp ./target/wasm32-unknown-unknown/release/contract.wasm ./res/
    ```
以上操作已经封装在 makefile 文件中
    ```shell
   make all
    ```

#### 合约交互
1. 部署并初始化合约  
    ➔ 合约账户：contract1234501.testnet  
    ➔ near contract deploy contract1234501.testnet use-file ./res/contract.wasm with-init-call init json-args '{"owner_id":"owner123.testnet"}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/FFAB7bzLQsyyGbUn5XMZTLWN4rvbJA1npXHjuuzPmj6H  

    1.1 重新部署合约，覆盖原合约  
    ➔ 合约账户：contract1234501.testnet  
    ➔ near contract deploy contract1234501.testnet use-file ./res/contract.wasm without-init-call init json-args '{"owner_id":"owner123.testnet"}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/HQTcvycQ8CXd8YA5WtQ5FzNCvQbRan4ENaGGqdsnwaAK 

2. 调用 newCampaign 方法，创建募捐活动  
    ➔ 募捐活动接收账户：owner123.testnet  
    ➔ near contract call-function as-transaction contract1234501.testnet newCampaign json-args '{"theme":"涿州水灾募捐活动","receiver":"owner123.testnet","number_funders":0,"funding_goal":100}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as owner123.testnet network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/A2ZVJi8ihQrAtLY9pQUFFALdJWwC2LDbFBJnbv9xoXv4  
    
3. 调用 get_crowdFunding_by_num_campagins 方法，查看创建的募捐活动  
    ➔ 募捐活动编号：1  
    ➔ near contract call-function as-transaction contract1234501.testnet get_crowdFunding_by_num_campagins json-args '{"num_campagins":1}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as owner123.testnet network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/BA9Rj1EM1Q2fLMmFpbagJd8YiTUCa7KbMb8D11ujEr8H  
    
4. 调用 bid 方法, 参与募捐活动  
    ➔ 参与募捐活动账户：zhouzhou_near.testnet  
    ➔ 发起转账 from： zhouzhou_near.testnet to ：contract1234501.testnet ，NEAR 88, 
    near contract call-function as-transaction contract1234501.testnet bid json-args '{"num_campagins":1}' prepaid-gas '100.000 TeraGas' attached-deposit '88 NEAR' sign-as zhouzhou_near.testnet network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/F11L4erVatpe1JnKUk3GaXMrxN1aJEUvXKh3KaEx2h4g  
    
5. 调用 get_funders_by_num_campagins 方法, 查看活动募捐人列表  
    ➔ near contract call-function as-transaction contract1234501.testnet get_funders_by_num_campagins json-args '{"num_campagins":1}' prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as owner123.testnet network-config testnet sign-with-keychain send  
    ➔ 合约浏览器：https://explorer.testnet.near.org/transactions/7rcGezoSa1cxqB7uMzofok9iynxeJ1K2ThLCw7VoG2fh  
    
##### 合约所有交易情况列表：  
    ➔ https://testnet.nearblocks.io/zh-cn/address/contract1234501.testnet  
        

## near-api-js
### 执行命令
```shell
yarn start
```
### 日志log  
▶▶▶▶ 募捐活动编号:40  
▶▶▶▶ 创建募捐活动:'募捐活动编号:40,募捐活动名称:流浪动物救助募捐活动, 募捐收款账号:owner123.testnet, 目标募捐金额:500 NEAR, 参与募捐人数:0, 实际募捐金额:0 yocto (单位：1NEAR = 10^24yoctoNEAR，1NEAR = 10^12Tera)'  
▶▶▶▶ '第1位募捐人'  
▶▶▶▶ 捐赠后募捐活动:'募捐活动编号:40,募捐活动名称:流浪动物救助募捐活动, 募捐收款账号:owner123.testnet, 目标募捐金额:500 NEAR, 参与募捐人数:1, 实际募捐金额:8e+24 yocto (单位：1NEAR = 10^24yoctoNEAR，1NEAR = 10^12Tera)'  
▶▶▶▶ 募捐活动编码:40, 捐赠人:[ { addr: 'zhouzhou_near.testnet', amount: 8e+24 }, [length]: 1   