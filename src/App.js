import "./App.css";
import React from 'react';
// import BN from 'bn.js';
import * as nearAPI from 'near-api-js'
import Big from 'big.js';

const { keyStores, KeyPair, connect, utils} = nearAPI;

interface CrowdFunding {
  num_campagins: number;  //活动编号
  theme: string;
  receiver: string;
  funding_goal: number;
  number_funders: number;
  total_amount: number;
} 

interface Funder {
  num_campagins: number;
  addr: string;
  amount: number;
} 

const TGas = Big(10).pow(12);
const BoatOfGas = Big(200).mul(TGas);

const ContractName = 'contract1234501.testnet';

class App extends React.Component {
  constructor(props) {
    super(props);

    /**
     * 上下文信息（合约信息，页面信息等）
     */
    this.state = {
      connected: false, //
      signedIn: false,  //是否已经登陆钱包
      accountId: null,  //募捐活动合约授权账户（只有此账户才有权创建募捐活动）

      //contract_accountId: "contract1234501.testnet",  //合约账户（合约已经部署到NEAR测试链上，合约授权账户为：owner123.testnet）
      contract_sign_accountId: "owner123.testnet",  //合约授权账户

      //创建募捐合同
      new_crowdfunding_theme:"",
      new_crowdfunding_receiver:"",
      new_crowdfunding_funding_goal:0, 

      //募捐活动信息
      crowdFunding_campagins: new Array(),
      crowdFunding_funding_map : new Map(), //key：活动编号；value：参与人列表
      crowdFunding_fundings : new Array(), 

      //
      crowdfunding_num:0,
      crowdfunding_goal:0,
    };

    this.handleNewCrowdChange = this.handleNewCrowdChange.bind(this);
    this.handleNewReceiverChange = this.handleNewReceiverChange.bind(this);
    this.handleNewFundingGoalChange = this.handleNewFundingGoalChange.bind(this);
    this.createNewCrowdSubmit = this.createNewCrowdSubmit.bind(this);
    this.sendCrowdFundGoalSubmit = this.sendCrowdFundGoalSubmit.bind(this);


    this._initNear().then(() => {
      this.setState({
        connected: true,
        signedIn: !!this._accountId,
        accountId: this._accountId,
      });
    }); 
  }

  handleNewCrowdChange(event) {
    console.log("募捐活动名称：" + event.target.value);
    this.setState({new_crowdfunding_theme: event.target.value});
  }

  handleNewReceiverChange(event) {
    console.log("募捐活动接收账户：" + event.target.value);
    this.setState({new_crowdfunding_receiver: event.target.value});
  }

  handleNewFundingGoalChange(event) {
    console.log("募捐活动筹集目标金额：" + event.target.value);
    this.setState({new_crowdfunding_funding_goal: event.target.value});
  }

  handlewCrowdFundingNumChange(event) {
    console.log("参与募捐活动的编号：" + event.target.value);
    this.setState({crowdfunding_num: event.target.value});
  }

  handlewCrowdFundingGoalChange(event) {
    console.log("参与募捐的金额：" + event.target.value);
    this.setState({crowdfunding_goal: event.target.value});
  }

  createNewCrowdSubmit(event) {
    this.createNewCrowd();
    event.preventDefault();
  }

  async createNewCrowd() {
    let num_campagins = await this._contract.newCampaign({"theme":this.state.new_crowdfunding_theme, "receiver":this.state.new_crowdfunding_receiver, "number_funders":0, "funding_goal":Number(this.state.new_crowdfunding_funding_goal)});
    if(num_campagins > 0 ) {
      alert("合约创建成功，合约编号：" + num_campagins);
      if (this._accountId) {
        await this.refreshAccountStats();
      }
    }
  }

  sendCrowdFundGoalSubmit(event) {
    this.bidCrowdFund();
    event.preventDefault();
  }

  async bidCrowdFund() {
    let funder_number = await this._account.functionCall({
                                                    contractId: ContractName,
                                                    methodName: "bid",
                                                    args: {
                                                      num_campagins: Number(this.state.crowdfunding_num),
                                                    },
                                                    gas: "300000000000000",
                                                    attachedDeposit: utils.format.parseNearAmount(this.state.crowdfunding_goal),
                                                  });
    //let funder_number = await this._contract.bid({"num_campagins":Number(this.state.crowdfunding_num)});
    if(funder_number > 0 ) {
      console.log("第：" + funder_number + "位捐赠人");
      if (this._accountId) {
        await this.refreshAccountStats();
      }
    }
  }

  async _initNear() {
    const nearConfig = {
      networkId: 'default',
      nodeUrl: 'https://rpc.testnet.near.org',
      contractName: ContractName,
      walletUrl: 'https://wallet.testnet.near.org',
    };
    const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
    const near = await nearAPI.connect(Object.assign({ deps: { keyStore } }, nearConfig));
    this._keyStore = keyStore;
    this._nearConfig = nearConfig;
    this._near = near;

    this._walletConnection = new nearAPI.WalletConnection(near, ContractName);
    this._accountId = this._walletConnection.getAccountId();
    this._account = this._walletConnection.account();
    //
    console.log("_accountId=" + this._accountId + ", _account=" + this._account);
    //
    
    this._contract = new nearAPI.Contract(this._account, ContractName, {
      viewMethods: ['get_crowdFunding_by_num_campagins', 'get_all_crowdFunding', 'get_funders_by_num_campagins'],
      changeMethods: ['newCampaign', 'bid'],
    });

    if (this._accountId) {
      await this.refreshAccountStats();
    }
  }

  async refreshAccountStats() {
    let crowdFunding_campagin_arry:CrowdFunding[]|undefined|null = await this._contract.get_all_crowdFunding();
    for(var i=0; i<crowdFunding_campagin_arry.length; i++){
      let crowdFunding_campagin_item = crowdFunding_campagin_arry[i];
      crowdFunding_campagin_item.num_campagins = i + 1;
      this.state.crowdFunding_campagins.push(crowdFunding_campagin_item);
      //
      let crowdFunding_funding_arry:Funder[]|undefined|null = await this._contract.get_funders_by_num_campagins({num_campagins: crowdFunding_campagin_item.num_campagins});
      if(crowdFunding_funding_arry.length > 0 ) {
        this.state.crowdFunding_funding_map.set(crowdFunding_campagin_item.num_campagins, crowdFunding_funding_arry);
      }
    }
  }

  async requestSignIn() {
    const appTitle = '去中心化募捐平台';
    await this._walletConnection.requestSignIn(
      ContractName,
      appTitle
    )
  }

  async logOut() {
    this._walletConnection.signOut();
    this._accountId = null;
    this.setState({
      signedIn: !!this._accountId,
      accountId: this._accountId,
    })
  }

  render() {
    console.log("call render().");
    const content = !this.state.connected ? (
      <div>
        正在链接NEAR钱包 ...
        <span className="spinner-grow spinner-grow-sm" role="status" aria-hidden="true"></span>
      </div>
    ) : (this.state.signedIn ? (
      <div>
        <div className="float-right">
          <button className="btn btn-outline-secondary" onClick={() => this.logOut()}>退出</button>
        </div>
        <h5><span className="font-weight-bold">当前登陆NEAR账号：{this.state.accountId}</span></h5>
        <div>
      </div>
      {
        this.state.contract_sign_accountId === this.state.accountId ? 
        <div>
          <h6><span className="font-weight-bold">您有权创建募捐活动</span></h6>
          <hr></hr>
          <label style={{marginTop:"10px"}}> 〓 创建募捐活动〓 </label>
          <form onSubmit={this.createNewCrowdSubmit}>
            <label>活动名称： <input type="text" style={{marginLeft:"10px"}} value={this.state.new_crowdfunding_theme} onChange={(event) => {this.handleNewCrowdChange(event)}} /> </label><br/>
            <label>接收募捐账户：<input type="text" style={{marginLeft:"10px"}} value={this.state.new_crowdfunding_receiver} onChange={(event) => {this.handleNewReceiverChange(event)}} /> </label><br/>
            <label>募捐目标金额： <input type="number" style={{marginLeft:"10px"}} value={this.state.new_crowdfunding_funding_goal} onChange={(event) => {this.handleNewFundingGoalChange(event)}} /> </label><br/>
            <input type="submit" value="提交" />
          </form>

          <hr></hr>
          <div style={{fontSize: '16px'}}>
          <label> 〓 已创建的募捐活动列表 〓 </label> <br/>
            {
                this.state.crowdFunding_campagins.map(item=>(
                    <li key={item} >『编号』：{item.num_campagins +", "} 『活动名称』：{item.theme +", "} 『募捐接收账户』：{item.receiver + ", " } 『募资目标金额』：{item.funding_goal + "NEAR, " } 『募资参与人数』：{item.number_funders + ", "}  『已经募集的金额』：{item.total_amount + " yoctoNEAR"} 
                      <br/>
                      <label style={{fontSize: '13px'}}>  ✿ 募捐人列表 </label> <br/>
                      <div style={{fontSize: '10px'}}>
                        {
                          this.state.crowdFunding_funding_map.get(item.num_campagins) != null && this.state.crowdFunding_funding_map.get(item.num_campagins) != undefined && this.state.crowdFunding_funding_map.get(item.num_campagins).length > 0 ? (
                            this.state.crowdFunding_funding_map.get(item.num_campagins).map(item12=>(
                              <div key={item12}> ◦  捐赠人地址：{item12.addr +", "} 捐赠金额：{item12.amount + " yoctoNEAR"} <br/></div>
                            )  
                          )
                          ):(
                              <label> 无捐赠人.</label> 
                          )
                        }
                      </div>
                    </li>
                  ))
            }
          </div>
          <br/>
          
          <hr></hr>
        </div>
        
       : 
       
        <div>
          <h6><span className="font-weight-bold">欢迎您参与募捐活动</span></h6>
          <hr></hr>
          <label> 请选择您要参与的募捐活动 </label> <br/>
          {
            (
            this.state.crowdFunding_campagins.map(item=>(
              <label>
                『编号』: {item.num_campagins +", "} 『活动名称』: {item.theme +", "} 『募捐接收账户』: {item.receiver + ", " } 『募捐人数』: {item.number_funders}
                <br/>
              </label>
            ))
            )
          }
          <hr></hr>
          <form onSubmit={this.sendCrowdFundGoalSubmit}>
            <label>活动编号： <input type="text" style={{marginLeft:"10px"}} value={this.state.crowdfunding_num} onChange={(event) => {this.handlewCrowdFundingNumChange(event)}} /> (请选择上面的活动编号)</label><br/>
            <label>募捐金额： <input type="number" style={{marginLeft:"10px"}} value={this.state.crowdfunding_goal} onChange={(event) => {this.handlewCrowdFundingGoalChange(event)}} /> </label><br/>
            <input type="submit" value="提交" />
          </form>
        </div>
      }
      </div>
    ) : (
      <div style={{ marginBottom: "10px" }}>
        <button
          className="btn btn-primary"
          onClick={() => this.requestSignIn()}>登陆NEAR钱包</button>
      </div>
    ));
    return (
        <div className="px-5">
          <h1>NEAR去中心化募捐活动平台</h1>
          <span className="font-weight-light">(提示⚠️：募捐活动合约已经部署到测试链，合约账户：{ContractName}，授权账户：{this.state.contract_sign_accountId})</span>
          {content}
        </div>
    );
  }
}

export default App;
