use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; // self 必须导入

use near_sdk::serde::Serialize;
use near_sdk::store::UnorderedMap;
use near_sdk::store::Vector;
use near_sdk::CryptoHash;
use std::iter::FromIterator;

use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

/*
 * 创建不同的募资活动，用来募集NEAR
 * 记录相应活动下的募资总体信息（参与人数，募集的NEAR数量），以及记录参与的用户地址以及投入的数量
 * 业务逻辑（用户参与，添加新的募集活动，活动结束后进行资金领取）
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] // 实现 borsh 序列化, 实现不可用的 `default` 方法以通过编译
pub struct Contract {
    //合约账户，用于接收募捐的NEAR
    owner_id: AccountId,
    //最新活动编号
    num_campagins: u32,
    //K 活动编号，V 募资活动
    campaigns: UnorderedMap<u32, CrowdFunding>,
    //K 活动编号，V 参与人
    funders: UnorderedMap<u32, Vector<Funder>>,
}

// near-sdk 提供的容器在初始化的时候都需要唯一的 storage key
// 可以使用 `#[derive(BorshStorageKey)]` 宏来获取 storage key. 它将枚举值按顺序以 `u8` 的方式进行 borsh 序列化, 最多可以得到 256 种不同的 storage key
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    // 以 0u8 的方式 borsh 序列化
    campaigns_storagekey,
    funders_storagekey,
    // 以 1u8 的方式 borsh 序列化
    #[allow(unused)]
    DynamicKey {
        num_campagins_storagekey: u32,
    }
}

// 使用 Default 来初始化合约
// impl Default for Contract {
//     fn default() -> Self {
//         unimplemented!();
//     }
// }

/*
 * 募资活动
 */
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CrowdFunding {
    theme: String,           //募捐活动主题
    pub receiver: AccountId, //接收募捐的NEAR账户地址
    funding_goal: u32,       //募资目标金额
    number_funders: u32,     //募资参与人数
    total_amount: u128,      //当前已经募集的金额
}

impl CrowdFunding {
    pub fn new(
        theme: String,
        receiver: AccountId,
        number_funders: u32,
        funding_goal: u32,
    ) -> CrowdFunding {
        let total_amount = 0;
        CrowdFunding {
            theme,
            receiver,
            number_funders,
            funding_goal,
            total_amount,
        }
    }
}
/*
 * 募资参与人信息
 */
#[derive(BorshDeserialize, BorshSerialize, Serialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct Funder {
    addr: AccountId,
    amount: u128,
}

impl Funder {
    pub fn new(addr: AccountId, amount: u128) -> Funder {
        let _total_amount = 0;
        Self { addr, amount }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            num_campagins: 0,
            campaigns: UnorderedMap::new(StorageKey::campaigns_storagekey),
            funders: UnorderedMap::new(StorageKey::funders_storagekey),
        }
    }

    //创建募捐活动，返回募集活动id
    pub fn newCampaign(
        &mut self,
        theme: String,
        receiver: AccountId,
        number_funders: u32,
        funding_goal: u32,
    ) -> Option<u32> {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );

        self.num_campagins += 1;
        let crowd_funding = CrowdFunding::new(theme, receiver, number_funders, funding_goal);
        self.campaigns.insert(self.num_campagins, crowd_funding);
        //
        self.funders
            .insert(self.num_campagins, Vector::new(StorageKey::DynamicKey { 
                                                                            num_campagins_storagekey : self.num_campagins
                                                                        }));

        Some(self.num_campagins)
    }

    //用户参与募捐活动, 返回此募捐活动的第几位参与者
    #[payable]
    pub fn bid(&mut self, num_campagins: u32) -> u32 {
        let opt_crowd_funding = self.campaigns.get_mut(&num_campagins);

        //todo 如果活动不存在 暂不考虑
        let crowd_funding: &mut CrowdFunding = opt_crowd_funding.unwrap();
        //参与人募捐的near数量
        let amount = env::attached_deposit();
        crowd_funding.total_amount += amount;
        crowd_funding.number_funders += 1_u32;
        //
        let opt_funders = self.funders.get_mut(&num_campagins);
        let funders: &mut Vector<Funder> = opt_funders.unwrap();
        //
        let funder_account_id = env::signer_account_id();
        let funder = Funder::new(funder_account_id, amount);
        funders.push(funder);

        funders.len()
    }

    // 获取募捐合约所属人
    pub fn get_contract_owner_id(&self) -> Option<&AccountId> {
        Some(&self.owner_id)
    }

    // 获取募捐活动by募捐活动编号
    pub fn get_crowdFunding_by_num_campagins(&self, num_campagins: u32) -> Option<&CrowdFunding> {
        if let Some(crowd_funding) = self.campaigns.get(&num_campagins) {
            Some(crowd_funding)
        } else {
            None
        }
    }

    // 获取所有募捐活动
    pub fn get_all_crowdFunding(&self) -> Option<Vec<&CrowdFunding>> {
        let crowdFunding_vec = Vec::from_iter(self.campaigns.values());
        Some(crowdFunding_vec)
    }

    // #[handle_result]
    // pub fn get_crowdFunding_by_num_campagins(&self, num_campagins: u32) -> Result<CrowdFunding, String> {
    //     if let Some(crowd_funding) = self.campaigns.get(&num_campagins) {
    //         Ok(crowd_funding.clone())
    //     } else {
    //         Err("募捐活动不存在".to_string())
    //     }
    // }

    //获取募捐人by募捐活动编号
    pub fn get_funders_by_num_campagins(&self, num_campagins: u32) -> Option<Vec<&Funder>> {
        let opt_funders = self.funders.get(&num_campagins);
        if opt_funders.is_none() {
            None
        } else {
            let mut funder_vec = Vec::new();
            let funders = opt_funders.unwrap();
            for funder in funders {
                funder_vec.push(funder);
            }

            Some(funder_vec)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Contract, CrowdFunding};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, Balance};

    //合约账户
    fn contract_owner() -> AccountId {
        "contract_owner.near".parse().unwrap()
    }

    //募捐活动接受账户1
    fn campaign_receiver1() -> AccountId {
        "campaign_receiver1.near".parse().unwrap()
    }
    fn campaign_receiver2() -> AccountId {
        "campaign_receiver2.near".parse().unwrap()
    }

    fn funder_addr1() -> AccountId {
        "funder_addr1.near".parse().unwrap()
    }
    fn funder_addr2() -> AccountId {
        "funder_addr2.near".parse().unwrap()
    }
    fn funder_addr3() -> AccountId {
        "funder_addr3.near".parse().unwrap()
    }

    const ONE_TOKEN: Balance = 1_000_000_000_000_000_000;

    #[test]
    fn test_init_process() {
        //
        let context = VMContextBuilder::new()
            .predecessor_account_id(contract_owner())
            .signer_account_id(funder_addr1())
            .attached_deposit(8)
            .build();
        testing_env!(context);

        /*
         * step1 创建募捐合约
         */
        let mut contract = Contract::init(contract_owner());
        let owner_id: Option<&AccountId> = contract.get_contract_owner_id();
        println!("owner_id: {:?}", owner_id);
        assert_eq!(owner_id.unwrap().as_str(), "contract_owner.near");

        /*
         * step2 创建2个募捐活动
         */
        //创建第1个募捐活动，目标筹集：100NEAR, 接收账户：campaign_receiver1.near
        let num_campaign1: Option<u32> = contract.newCampaign(
            String::from("涿州水灾募捐活动"),
            campaign_receiver1(),
            0_u32,
            100_u32,
        );
        //
        assert_eq!(num_campaign1.unwrap(), 1, "第1个募捐活动编号错误！");

        let opt_crowd_funding1 = contract.get_crowdFunding_by_num_campagins(num_campaign1.unwrap());
        let crowd_funding1 = opt_crowd_funding1.unwrap();
            assert_eq!(
                crowd_funding1.receiver.as_str(),
                "campaign_receiver1.near",
                "第1个募捐活动，接收账户错误！"
            );

        //创建第2个募捐活动，目标筹集：500NEAR, 接收账户：campaign_receiver2.near
        let num_campaign2: Option<u32> = contract.newCampaign(
            String::from("流浪动物救助募捐活动"),
            campaign_receiver2(),
            0_u32,
            500_u32,
        );
        //
        assert_eq!(num_campaign2.unwrap(), 2, "第2个募捐活动编号错误！");

        let opt_crowd_funding2 = contract.get_crowdFunding_by_num_campagins(num_campaign2.unwrap());
        let crowd_funding2 = opt_crowd_funding2.unwrap();
            assert_eq!(
                crowd_funding2.receiver.as_str(),
                "campaign_receiver2.near",
                "第2个募捐活动，接收账户错误！"
            );

        /*
         * step3 2个用户参与第1个募捐活动，
         */
        let funder_number = contract.bid(num_campaign1.unwrap());
        assert_eq!(funder_number, 1, "用户参与募捐活动1失败！");
        if funder_number > 0 {
            let funders = contract
                .get_funders_by_num_campagins(num_campaign1.unwrap())
                .unwrap();
            assert_eq!(funders.len(), 1);
            let v_iter = funders.iter();
            for val in v_iter {
                assert_eq!(val.addr, funder_addr1());
            }
        }
        //模拟第2个用户
        let context1 = VMContextBuilder::new()
            .predecessor_account_id(contract_owner())
            .signer_account_id(funder_addr2())
            .attached_deposit(9)
            .build();
        testing_env!(context1);
        //
        let funder_number = contract.bid(num_campaign1.unwrap());
        assert_eq!(funder_number, 2, "用户参与募捐活动1失败！");

        //模拟第3个用户
        let context2 = VMContextBuilder::new()
            .predecessor_account_id(contract_owner())
            .signer_account_id(funder_addr3())
            .attached_deposit(50)
            .build();
        testing_env!(context2);
        //
        let funder_number = contract.bid(num_campaign2.unwrap());
        assert_eq!(funder_number, 1, "用户参与募捐活动2失败！");
        /**
         * step4 查看所有募捐活动
         */
        let crowdFunding_vec = contract.get_all_crowdFunding();
        assert_eq!(crowdFunding_vec.unwrap().len(), 2, "募捐活动个数不对！");

        /*
         * step5 查看所有募捐活动及参与人
         */
        let owner_id = contract.owner_id;
        println!("合约账号：{:?}", owner_id.as_str());
        let campaigns = contract.campaigns;
        let campaigns_iter = campaigns.iter();
        for campaign in campaigns_iter {
            println!("【{}】合约接收账户：{}, 目标募集数量：{}near, 参与人数：{}人, 当前已筹集：{} yocto.", campaign.1.theme, campaign.1.receiver, campaign.1.funding_goal, campaign.1.number_funders, campaign.1.total_amount);
            let opt_funders = contract.funders.get(campaign.0);
            if opt_funders.is_some() {
                let funders = opt_funders.unwrap().iter();
                for funder in funders {
                    println!(
                        "     --->【募捐人】账户：{}, 捐赠：{} yocto.",
                        funder.addr, funder.amount
                    );
                }
            }
            println!()
        }
    }
}
