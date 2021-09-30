import "./App.css";
import React from 'react';
// import BN from 'bn.js';
import * as nearAPI from 'near-api-js'
import Big from 'big.js';

const TGas = Big(10).pow(12);
const BoatOfGas = Big(200).mul(TGas);



const ContractName = 'dev-1632905384478-86340764291081';

class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      connected: false,
      signedIn: false,
      accountId: null,
      
      one_animal: "",
      two_animal: "",
      levelup_target:"",
      animal_name:"",
      new_animal:"",
      owe_animal:"",
      owe_level:""
    };

    this._digitalRefreshTimer = null;


    this.handleLevelUpTargetChange = this.handleLevelUpTargetChange.bind(this);
    this.levelupSubmit = this.levelupSubmit.bind(this);
    this.addfirstSubmit = this.addfirstSubmit.bind(this);
    this.handlenewChange = this.handlenewChange.bind(this);

    this.handleOwnChange = this.handleOwnChange.bind(this);
    this.handleTargetChange = this.handleTargetChange.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
    this.handleremove = this.handleremove.bind(this);

    this._initNear().then(() => {
      this.setState({
        connected: true,
        signedIn: !!this._accountId,
        accountId: this._accountId,
      });
    });
  }

  componentDidMount() {
  }




  async refreshAccountStats() {
    // let  animal_name = "adfdd";
    // alert(this._accountId)
    let animal_name = await this._contract.get_all();
    let animal_level = await this._contract.get_level({animal_name:animal_name})

    alert(animal_level)

    
    this.setState({
    owe_animal:animal_name,
    owe_level:animal_level
    });


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
    
    this._contract = new nearAPI.Contract(this._account, ContractName, {
      viewMethods: [],
      changeMethods: ['add_first',  'levelup', 'get_all','get_level','remove_animal','fight_with'],
    });

    // alert(this._nearConfig.contractName);
    // let animal_name = await this._contract.get_all({ accountId: this._accountId });
    if (this._accountId) {
      await this.refreshAccountStats();
    }
  }


  async requestSignIn() {
    const appTitle = '宠物大战';
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

  async addFirst() {
    let res = await this._contract.add_first({animal_name: this.state.new_animal});
    // let one = await this._contract.get_all();
    
    await this.refreshAccountStats();
  }

  async remove_action(){
    let res = await this._contract.remove_animal();
    alert(res)
    window.location.reload()
  }
  async pk_action(){
    let res = await this._contract.fight_with({my_animal: this.state.one_animal, animal_name: this.state.two_animal})
    alert(res)
    // window.location.reload()
  }
  handleOwnChange(event) {    this.setState({one_animal: event.target.value});  }
  handlenewChange(event) {    this.setState({new_animal: event.target.value});}
  // handlenewChange(event) {    this.setState({animal_name: event.target.value});}
  handleTargetChange(event) {    this.setState({two_animal: event.target.value});  }

  handleSubmit(event) {
    this.pk_action();
    event.preventDefault();
  }

  handleremove(event){
    this.remove_action();
    event.preventDefault();
  }

  async action_levelup(){
    let res = await this._contract.levelup({animal_name: this.state.animal_name}, BoatOfGas.toFixed(0), Big(10000000000).mul(TGas).toFixed(0));
    alert(res)
    window.location.reload()
  }

  handleLevelUpTargetChange(event) {this.setState({animal_name: event.target.value});  }

  levelupSubmit(event) {
    this.action_levelup();
    event.preventDefault();
  }
  addfirstSubmit(event) {
    this.addFirst();
    event.preventDefault();
  }

  render() {
    const content = !this.state.connected ? (
      <div>Connecting... <span className="spinner-grow spinner-grow-sm" role="status" aria-hidden="true"></span></div>
    ) : (this.state.signedIn ? (
      <div>
        <div className="float-right">
          <button
            className="btn btn-outline-secondary"
            onClick={() => this.logOut()}>Log out</button>
        </div>
        <h4><span className="font-weight-bold">{this.state.accountId}</span>!</h4>
        <div>
          
          

        </div>
        <form onSubmit={this.addfirstSubmit}>
          
        <label> <input type="text" style={{marginLeft:"10px"}} value={this.state.new_animal} onChange={(event) => {this.handlenewChange(event)}} /> </label>
          <input type="submit" value="领养宠物" />
        </form>
        <form onSubmit={this.levelupSubmit}>
          
        <label> <input type="text" style={{marginLeft:"10px"}} value={this.state.animal_name} onChange={(event) => {this.handleLevelUpTargetChange(event)}} /> </label>
          <input type="submit" value="升级宠物" />
        </form>
        <hr></hr>
        <form onSubmit={this.handleSubmit}>
          <label> 我方：<input type="text" style={{marginLeft:"44px"}} value={this.state.one_animal} onChange={(event) => {this.handleOwnChange(event)}} /> </label> <br/>
          <label> 敌方：<input type="text" style={{marginLeft:"44px"}} value={this.state.two_animal} onChange={(event) => {this.handleTargetChange(event)}} /> </label><br/>
          <input type="submit" value="挑战" />
          
        </form>

        <form onSubmit={this.handleremove}>
        <input type ="submit" style={{marginLeft:"20px"}} value = "安乐死 "/>
        </form>
        <input type ="submit" style={{marginLeft:"20px"}} value = "生成NFT "/>
        <hr></hr>

        <hr></hr>
        <div>
          你的宠物:
          <ul>
              <div>{this.state.owe_animal}</div>
              <div> {this.state.owe_level}</div>
          </ul>
        </div>
      </div>
    ) : (
      <div style={{ marginBottom: "10px" }}>
        <button
          className="btn btn-primary"
          onClick={() => this.requestSignIn()}>Log in with NEAR Wallet</button>
      </div>
    ));
    return (
      <div className="px-5">
        <h1>我的宠物</h1>
        {content}
      </div>
    //   <div style="display: flex">
    //   <input
    //     style="flex: 1"
    //     autocomplete="off"
    //     id="greeting"
    //     data-behavior="greeting"
    //   />
    //   <button disabled style="border-radius: 0 5px 5px 0">Save</button>
    // </div>
    );
  }
}

export default App;
