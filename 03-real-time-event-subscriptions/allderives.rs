#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{path::Path, sync::Arc};
use alloy_network::EthereumWallet;
use alloy_primitives::U256;
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::SolEventInterface;
use eyre::Result;
use url::Url;
use alloy_rpc_types::BlockNumberOrTag;
use SampleContract::SampleContractEvents;
use futures::StreamExt;
use tokio::sync::broadcast::{channel, Receiver, Sender};
///Module containing a contract's types and functions.
/**

```solidity
contract SampleContract {
    event ValueChanged(address indexed updater, uint256 indexed oldValue, uint256 newValue);
    event EtherReceived(address indexed sender, uint256 amount, uint256 newBalance);
    event EtherWithdrawn(address indexed recipient, uint256 amount, uint256 remainingBalance);
    error SampleError(string cause);
    constructor(uint256 _initialValue);
    function setValue(uint256 _value) external;
    function getValue() external view returns (uint256 currentValue);
    function deposit() external payable;
    function withdraw() external;
    function getBalance() external view returns (uint256 balance);
    function revertWithError() external pure;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod SampleContract {
    use super::*;
    use ::alloy_sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///608034604d57601f61028038819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f5560405161021a90816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c90816312065fe0146101cc5750806320965255146101405780633ccfd60b1461015c5780633fa4f2451461014057806355241077146100f857806357eca1a5146100ad5763d0e30db014610069575f80fd5b5f3660031901126100a957476040519034825260208201527f1d57945c1033a96907a78f6e0ebf6a03815725dac25f33cc806558670344ac8860403392a2005b5f80fd5b346100a9575f3660031901126100a9576040516335fdd7ab60e21b815260206004820152601260248201527168656c6c6f2066726f6d207265766572742160701b6044820152606490fd5b346100a95760203660031901126100a9576004355f5490805f556040519081527fe435f0fbe584e62b62f48f4016a57ef6c95e4c79f5babbe6ad3bb64f3281d26160203392a3005b346100a9575f3660031901126100a95760205f54604051908152f35b346100a9575f3660031901126100a95747805f81156101c3575b5f80809381933390f1156101b8576040519081525f60208201527fd5ca65e1ec4f4864fea7b9c5cb1ec3087a0dbf9c74641db3f6458edf445c405160403392a2005b6040513d5f823e3d90fd5b506108fc610176565b346100a9575f3660031901126100a957602090478152f3fea2646970667358221220cae439afc02e7259cc99c579d322222052f82f79b377ffd437d0523157cb795f64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x804`MW`\x1fa\x02\x808\x81\x90\x03\x91\x82\x01`\x1f\x19\x16\x83\x01\x91`\x01`\x01`@\x1b\x03\x83\x11\x84\x84\x10\x17`QW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12`MWQ_U`@Qa\x02\x1a\x90\x81a\0f\x829\xf3[_\x80\xfd[cNH{q`\xe0\x1b_R`A`\x04R`$_\xfd\xfe`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xfd[_5`\xe0\x1c\x90\x81c\x12\x06_\xe0\x14a\x01\xccWP\x80c \x96RU\x14a\x01@W\x80c<\xcf\xd6\x0b\x14a\x01\\W\x80c?\xa4\xf2E\x14a\x01@W\x80cU$\x10w\x14a\0\xf8W\x80cW\xec\xa1\xa5\x14a\0\xadWc\xd0\xe3\r\xb0\x14a\0iW_\x80\xfd[_6`\x03\x19\x01\x12a\0\xa9WG`@Q\x904\x82R` \x82\x01R\x7f\x1dW\x94\\\x103\xa9i\x07\xa7\x8fn\x0e\xbfj\x03\x81W%\xda\xc2_3\xcc\x80eXg\x03D\xac\x88`@3\x92\xa2\0[_\x80\xfd[4a\0\xa9W_6`\x03\x19\x01\x12a\0\xa9W`@Qc5\xfd\xd7\xab`\xe2\x1b\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rqhello from revert!`p\x1b`D\x82\x01R`d\x90\xfd[4a\0\xa9W` 6`\x03\x19\x01\x12a\0\xa9W`\x045_T\x90\x80_U`@Q\x90\x81R\x7f\xe45\xf0\xfb\xe5\x84\xe6+b\xf4\x8f@\x16\xa5~\xf6\xc9^Ly\xf5\xba\xbb\xe6\xad;\xb6O2\x81\xd2a` 3\x92\xa3\0[4a\0\xa9W_6`\x03\x19\x01\x12a\0\xa9W` _T`@Q\x90\x81R\xf3[4a\0\xa9W_6`\x03\x19\x01\x12a\0\xa9WG\x80_\x81\x15a\x01\xc3W[_\x80\x80\x93\x81\x933\x90\xf1\x15a\x01\xb8W`@Q\x90\x81R_` \x82\x01R\x7f\xd5\xcae\xe1\xecOHd\xfe\xa7\xb9\xc5\xcb\x1e\xc3\x08z\r\xbf\x9ctd\x1d\xb3\xf6E\x8e\xdfD\\@Q`@3\x92\xa2\0[`@Q=_\x82>=\x90\xfd[Pa\x08\xfca\x01vV[4a\0\xa9W_6`\x03\x19\x01\x12a\0\xa9W` \x90G\x81R\xf3\xfe\xa2dipfsX\"\x12 \xca\xe49\xaf\xc0.rY\xcc\x99\xc5y\xd3\"\" R\xf8/y\xb3w\xff\xd47\xd0R1W\xcby_dsolcC\0\x08\x1b\03",
    );
    /**Event with signature `ValueChanged(address,uint256,uint256)` and selector `0xe435f0fbe584e62b62f48f4016a57ef6c95e4c79f5babbe6ad3bb64f3281d261`.
```solidity
event ValueChanged(address indexed updater, uint256 indexed oldValue, uint256 newValue);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct ValueChanged {
        #[allow(missing_docs)]
        pub updater: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldValue: ::alloy_sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newValue: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for ValueChanged {
        #[inline]
        fn clone(&self) -> ValueChanged {
            ValueChanged {
                updater: ::core::clone::Clone::clone(&self.updater),
                oldValue: ::core::clone::Clone::clone(&self.oldValue),
                newValue: ::core::clone::Clone::clone(&self.newValue),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValueChanged {
            type DataTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                ::alloy_sol_types::sol_data::Address,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "ValueChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                228u8,
                53u8,
                240u8,
                251u8,
                229u8,
                132u8,
                230u8,
                43u8,
                98u8,
                244u8,
                143u8,
                64u8,
                22u8,
                165u8,
                126u8,
                246u8,
                201u8,
                94u8,
                76u8,
                121u8,
                245u8,
                186u8,
                187u8,
                230u8,
                173u8,
                59u8,
                182u8,
                79u8,
                50u8,
                129u8,
                210u8,
                97u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    updater: topics.1,
                    oldValue: topics.2,
                    newValue: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newValue),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.updater.clone(),
                    self.oldValue.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.updater,
                );
                out[2usize] = <::alloy_sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.oldValue);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValueChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValueChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValueChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EtherReceived(address,uint256,uint256)` and selector `0x1d57945c1033a96907a78f6e0ebf6a03815725dac25f33cc806558670344ac88`.
```solidity
event EtherReceived(address indexed sender, uint256 amount, uint256 newBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct EtherReceived {
        #[allow(missing_docs)]
        pub sender: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newBalance: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for EtherReceived {
        #[inline]
        fn clone(&self) -> EtherReceived {
            EtherReceived {
                sender: ::core::clone::Clone::clone(&self.sender),
                amount: ::core::clone::Clone::clone(&self.amount),
                newBalance: ::core::clone::Clone::clone(&self.newBalance),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EtherReceived {
            type DataTuple<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                ::alloy_sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EtherReceived(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                29u8,
                87u8,
                148u8,
                92u8,
                16u8,
                51u8,
                169u8,
                105u8,
                7u8,
                167u8,
                143u8,
                110u8,
                14u8,
                191u8,
                106u8,
                3u8,
                129u8,
                87u8,
                37u8,
                218u8,
                194u8,
                95u8,
                51u8,
                204u8,
                128u8,
                101u8,
                88u8,
                103u8,
                3u8,
                68u8,
                172u8,
                136u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sender: topics.1,
                    amount: data.0,
                    newBalance: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EtherReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EtherReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EtherReceived) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EtherWithdrawn(address,uint256,uint256)` and selector `0xd5ca65e1ec4f4864fea7b9c5cb1ec3087a0dbf9c74641db3f6458edf445c4051`.
```solidity
event EtherWithdrawn(address indexed recipient, uint256 amount, uint256 remainingBalance);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    pub struct EtherWithdrawn {
        #[allow(missing_docs)]
        pub recipient: ::alloy_sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: ::alloy_sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub remainingBalance: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    impl ::core::clone::Clone for EtherWithdrawn {
        #[inline]
        fn clone(&self) -> EtherWithdrawn {
            EtherWithdrawn {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
                remainingBalance: ::core::clone::Clone::clone(&self.remainingBalance),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EtherWithdrawn {
            type DataTuple<'a> = (
                ::alloy_sol_types::sol_data::Uint<256>,
                ::alloy_sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                ::alloy_sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "EtherWithdrawn(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                213u8,
                202u8,
                101u8,
                225u8,
                236u8,
                79u8,
                72u8,
                100u8,
                254u8,
                167u8,
                185u8,
                197u8,
                203u8,
                30u8,
                195u8,
                8u8,
                122u8,
                13u8,
                191u8,
                156u8,
                116u8,
                100u8,
                29u8,
                179u8,
                246u8,
                69u8,
                142u8,
                223u8,
                68u8,
                92u8,
                64u8,
                81u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: topics.1,
                    amount: data.0,
                    remainingBalance: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.remainingBalance),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EtherWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EtherWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EtherWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Custom error with signature `SampleError(string)` and selector `0xd7f75eac`.
```solidity
error SampleError(string cause);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct SampleError {
        pub cause: ::alloy_sol_types::private::String,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for SampleError {
        #[inline]
        fn clone(&self) -> SampleError {
            SampleError {
                cause: ::core::clone::Clone::clone(&self.cause),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String,);
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SampleError> for UnderlyingRustTuple<'_> {
            fn from(value: SampleError) -> Self {
                (value.cause,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SampleError {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { cause: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SampleError {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SampleError(string)";
            const SELECTOR: [u8; 4] = [215u8, 247u8, 94u8, 172u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.cause,
                    ),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(uint256 _initialValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct constructorCall {
        pub _initialValue: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for constructorCall {
        #[inline]
        fn clone(&self) -> constructorCall {
            constructorCall {
                _initialValue: ::core::clone::Clone::clone(&self._initialValue),
            }
        }
    }
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._initialValue,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _initialValue: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._initialValue),
                )
            }
        }
    };
    /// @notice Sets a new value for the 'value' state variable
    /// @param _value The new value to be set
    /**Function with signature `setValue(uint256)` and selector `0x55241077`.
```solidity
function setValue(uint256 _value) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct setValueCall {
        pub _value: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for setValueCall {
        #[inline]
        fn clone(&self) -> setValueCall {
            setValueCall {
                _value: ::core::clone::Clone::clone(&self._value),
            }
        }
    }
    /// @notice Sets a new value for the 'value' state variable
    /// @param _value The new value to be set
    ///Container type for the return parameters of the [`setValue(uint256)`](setValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct setValueReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for setValueReturn {
        #[inline]
        fn clone(&self) -> setValueReturn {
            setValueReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: setValueCall) -> Self {
                    (value._value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setValueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _value: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setValueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setValueReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setValueCall {
            type Parameters<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setValueReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setValue(uint256)";
            const SELECTOR: [u8; 4] = [85u8, 36u8, 16u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <::alloy_sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._value),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /// @notice Retrieves the current value of the 'value' state variable
    /// @return currentValue The current value stored in 'value'
    /**Function with signature `getValue()` and selector `0x20965255`.
```solidity
function getValue() external view returns (uint256 currentValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct getValueCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for getValueCall {
        #[inline]
        fn clone(&self) -> getValueCall {
            getValueCall {}
        }
    }
    /// @notice Retrieves the current value of the 'value' state variable
    /// @return currentValue The current value stored in 'value'
    ///Container type for the return parameters of the [`getValue()`](getValueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct getValueReturn {
        pub currentValue: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for getValueReturn {
        #[inline]
        fn clone(&self) -> getValueReturn {
            getValueReturn {
                currentValue: ::core::clone::Clone::clone(&self.currentValue),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getValueCall> for UnderlyingRustTuple<'_> {
                fn from(value: getValueCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getValueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getValueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getValueReturn) -> Self {
                    (value.currentValue,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getValueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { currentValue: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValueCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValueReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValue()";
            const SELECTOR: [u8; 4] = [32u8, 150u8, 82u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /// @notice Accepts Ether deposits and logs the sender and amount
    /**Function with signature `deposit()` and selector `0xd0e30db0`.
```solidity
function deposit() external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct depositCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for depositCall {
        #[inline]
        fn clone(&self) -> depositCall {
            depositCall {}
        }
    }
    /// @notice Accepts Ether deposits and logs the sender and amount
    ///Container type for the return parameters of the [`deposit()`](depositCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct depositReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for depositReturn {
        #[inline]
        fn clone(&self) -> depositReturn {
            depositReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit()";
            const SELECTOR: [u8; 4] = [208u8, 227u8, 13u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /// @notice Withdraws the entire balance of the contract to the caller
    /**Function with signature `withdraw()` and selector `0x3ccfd60b`.
```solidity
function withdraw() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct withdrawCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for withdrawCall {
        #[inline]
        fn clone(&self) -> withdrawCall {
            withdrawCall {}
        }
    }
    /// @notice Withdraws the entire balance of the contract to the caller
    ///Container type for the return parameters of the [`withdraw()`](withdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct withdrawReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for withdrawReturn {
        #[inline]
        fn clone(&self) -> withdrawReturn {
            withdrawReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw()";
            const SELECTOR: [u8; 4] = [60u8, 207u8, 214u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /// @notice Retrieves the contract's current Ether balance
    /// @return balance The current balance of the contract in wei
    /**Function with signature `getBalance()` and selector `0x12065fe0`.
```solidity
function getBalance() external view returns (uint256 balance);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct getBalanceCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for getBalanceCall {
        #[inline]
        fn clone(&self) -> getBalanceCall {
            getBalanceCall {}
        }
    }
    /// @notice Retrieves the contract's current Ether balance
    /// @return balance The current balance of the contract in wei
    ///Container type for the return parameters of the [`getBalance()`](getBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct getBalanceReturn {
        pub balance: ::alloy_sol_types::private::primitives::aliases::U256,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for getBalanceReturn {
        #[inline]
        fn clone(&self) -> getBalanceReturn {
            getBalanceReturn {
                balance: ::core::clone::Clone::clone(&self.balance),
            }
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                ::alloy_sol_types::private::primitives::aliases::U256,
            );
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceReturn) -> Self {
                    (value.balance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { balance: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceReturn;
            type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalance()";
            const SELECTOR: [u8; 4] = [18u8, 6u8, 95u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /// @notice Reverts the transaction with a custom error message
    /// @dev Used to demonstrate custom error handling in Solidity
    /**Function with signature `revertWithError()` and selector `0x57eca1a5`.
```solidity
function revertWithError() external pure;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct revertWithErrorCall {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for revertWithErrorCall {
        #[inline]
        fn clone(&self) -> revertWithErrorCall {
            revertWithErrorCall {}
        }
    }
    /// @notice Reverts the transaction with a custom error message
    /// @dev Used to demonstrate custom error handling in Solidity
    ///Container type for the return parameters of the [`revertWithError()`](revertWithErrorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    pub struct revertWithErrorReturn {}
    #[automatically_derived]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    impl ::core::clone::Clone for revertWithErrorReturn {
        #[inline]
        fn clone(&self) -> revertWithErrorReturn {
            revertWithErrorReturn {}
        }
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use ::alloy_sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revertWithErrorCall> for UnderlyingRustTuple<'_> {
                fn from(value: revertWithErrorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revertWithErrorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revertWithErrorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: revertWithErrorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for revertWithErrorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revertWithErrorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revertWithErrorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revertWithError()";
            const SELECTOR: [u8; 4] = [87u8, 236u8, 161u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`SampleContract`](self) function calls.
    pub enum SampleContractCalls {
        setValue(setValueCall),
        getValue(getValueCall),
        deposit(depositCall),
        withdraw(withdrawCall),
        getBalance(getBalanceCall),
        revertWithError(revertWithErrorCall),
    }
    #[automatically_derived]
    impl SampleContractCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [18u8, 6u8, 95u8, 224u8],
            [32u8, 150u8, 82u8, 85u8],
            [60u8, 207u8, 214u8, 11u8],
            [85u8, 36u8, 16u8, 119u8],
            [87u8, 236u8, 161u8, 165u8],
            [208u8, 227u8, 13u8, 176u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SampleContractCalls {
        const NAME: &'static str = "SampleContractCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::setValue(_) => <setValueCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getValue(_) => <getValueCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit(_) => <depositCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdraw(_) => <withdrawCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBalance(_) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::revertWithError(_) => {
                    <revertWithErrorCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<SampleContractCalls>] = &[
                {
                    fn getBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <getBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::getBalance)
                    }
                    getBalance
                },
                {
                    fn getValue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <getValueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::getValue)
                    }
                    getValue
                },
                {
                    fn withdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <withdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::withdraw)
                    }
                    withdraw
                },
                {
                    fn setValue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <setValueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::setValue)
                    }
                    setValue
                },
                {
                    fn revertWithError(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <revertWithErrorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::revertWithError)
                    }
                    revertWithError
                },
                {
                    fn deposit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractCalls> {
                        <depositCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractCalls::deposit)
                    }
                    deposit
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::setValue(inner) => {
                    <setValueCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getValue(inner) => {
                    <getValueCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBalance(inner) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::revertWithError(inner) => {
                    <revertWithErrorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::setValue(inner) => {
                    <setValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValue(inner) => {
                    <getValueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit(inner) => {
                    <depositCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBalance(inner) => {
                    <getBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::revertWithError(inner) => {
                    <revertWithErrorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SampleContract`](self) custom errors.
    pub enum SampleContractErrors {
        SampleError(SampleError),
    }
    #[automatically_derived]
    impl SampleContractErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[215u8, 247u8, 94u8, 172u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SampleContractErrors {
        const NAME: &'static str = "SampleContractErrors";
        const MIN_DATA_LENGTH: usize = 64usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::SampleError(_) => {
                    <SampleError as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<SampleContractErrors>] = &[
                {
                    fn SampleError(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SampleContractErrors> {
                        <SampleError as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SampleContractErrors::SampleError)
                    }
                    SampleError
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::SampleError(inner) => {
                    <SampleError as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::SampleError(inner) => {
                    <SampleError as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SampleContract`](self) events.
    pub enum SampleContractEvents {
        ValueChanged(ValueChanged),
        EtherReceived(EtherReceived),
        EtherWithdrawn(EtherWithdrawn),
    }
    #[automatically_derived]
    impl SampleContractEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                29u8,
                87u8,
                148u8,
                92u8,
                16u8,
                51u8,
                169u8,
                105u8,
                7u8,
                167u8,
                143u8,
                110u8,
                14u8,
                191u8,
                106u8,
                3u8,
                129u8,
                87u8,
                37u8,
                218u8,
                194u8,
                95u8,
                51u8,
                204u8,
                128u8,
                101u8,
                88u8,
                103u8,
                3u8,
                68u8,
                172u8,
                136u8,
            ],
            [
                213u8,
                202u8,
                101u8,
                225u8,
                236u8,
                79u8,
                72u8,
                100u8,
                254u8,
                167u8,
                185u8,
                197u8,
                203u8,
                30u8,
                195u8,
                8u8,
                122u8,
                13u8,
                191u8,
                156u8,
                116u8,
                100u8,
                29u8,
                179u8,
                246u8,
                69u8,
                142u8,
                223u8,
                68u8,
                92u8,
                64u8,
                81u8,
            ],
            [
                228u8,
                53u8,
                240u8,
                251u8,
                229u8,
                132u8,
                230u8,
                43u8,
                98u8,
                244u8,
                143u8,
                64u8,
                22u8,
                165u8,
                126u8,
                246u8,
                201u8,
                94u8,
                76u8,
                121u8,
                245u8,
                186u8,
                187u8,
                230u8,
                173u8,
                59u8,
                182u8,
                79u8,
                50u8,
                129u8,
                210u8,
                97u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SampleContractEvents {
        const NAME: &'static str = "SampleContractEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ValueChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValueChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValueChanged)
                }
                Some(<EtherReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EtherReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EtherReceived)
                }
                Some(<EtherWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EtherWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EtherWithdrawn)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SampleContractEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ValueChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EtherReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EtherWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ValueChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EtherReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EtherWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use ::alloy_contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SampleContract`](self) contract instance.

See the [wrapper's documentation](`SampleContractInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SampleContractInstance<T, P, N> {
        SampleContractInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _initialValue: ::alloy_sol_types::private::primitives::aliases::U256,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SampleContractInstance<T, P, N>>,
    > {
        SampleContractInstance::<T, P, N>::deploy(provider, _initialValue)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _initialValue: ::alloy_sol_types::private::primitives::aliases::U256,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SampleContractInstance::<T, P, N>::deploy_builder(provider, _initialValue)
    }
    /**A [`SampleContract`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SampleContract`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    pub struct SampleContractInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<
        T: ::core::clone::Clone,
        P: ::core::clone::Clone,
        N: ::core::clone::Clone,
    > ::core::clone::Clone for SampleContractInstance<T, P, N> {
        #[inline]
        fn clone(&self) -> SampleContractInstance<T, P, N> {
            SampleContractInstance {
                address: ::core::clone::Clone::clone(&self.address),
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::clone::Clone::clone(&self._network_transport),
            }
        }
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SampleContractInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SampleContractInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SampleContractInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SampleContract`](self) contract instance.

See the [wrapper's documentation](`SampleContractInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _initialValue: ::alloy_sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::Result<SampleContractInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _initialValue);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _initialValue: ::alloy_sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { _initialValue },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> SampleContractInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SampleContractInstance<T, P, N> {
            SampleContractInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SampleContractInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`setValue`] function.
        pub fn setValue(
            &self,
            _value: ::alloy_sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setValueCall, N> {
            self.call_builder(&setValueCall { _value })
        }
        ///Creates a new call builder for the [`getValue`] function.
        pub fn getValue(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValueCall, N> {
            self.call_builder(&getValueCall {})
        }
        ///Creates a new call builder for the [`deposit`] function.
        pub fn deposit(&self) -> alloy_contract::SolCallBuilder<T, &P, depositCall, N> {
            self.call_builder(&depositCall {})
        }
        ///Creates a new call builder for the [`withdraw`] function.
        pub fn withdraw(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawCall, N> {
            self.call_builder(&withdrawCall {})
        }
        ///Creates a new call builder for the [`getBalance`] function.
        pub fn getBalance(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceCall, N> {
            self.call_builder(&getBalanceCall {})
        }
        ///Creates a new call builder for the [`revertWithError`] function.
        pub fn revertWithError(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, revertWithErrorCall, N> {
            self.call_builder(&revertWithErrorCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SampleContractInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ValueChanged`] event.
        pub fn ValueChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValueChanged, N> {
            self.event_filter::<ValueChanged>()
        }
        ///Creates a new event filter for the [`EtherReceived`] event.
        pub fn EtherReceived_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EtherReceived, N> {
            self.event_filter::<EtherReceived>()
        }
        ///Creates a new event filter for the [`EtherWithdrawn`] event.
        pub fn EtherWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EtherWithdrawn, N> {
            self.event_filter::<EtherWithdrawn>()
        }
    }
}
fn main() -> Result<()> {
    let body = async {
        let env_path = Path::new(
                "/Users/edoardoierina/Projects/rust/alloy-in-action/03-real-time-event-subscriptions",
            )
            .parent()
            .unwrap()
            .join(".env");
        dotenv::from_path(env_path).ok();
        tracing_subscriber::fmt::init();
        let private_key = std::env::var("ANVIL_PRIVATE_KEY")?;
        let signer: PrivateKeySigner = private_key.parse()?;
        let wallet: EthereumWallet = EthereumWallet::from(signer);
        let ws_url = std::env::var("ANVIL_WS_URL")?;
        let ws_url = Url::parse(&ws_url)?;
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_ws(WsConnect::new(ws_url))
            .await?;
        let provider = Arc::new(provider);
        let initial_value = U256::from(1);
        let contract = SampleContract::deploy(&provider, initial_value).await?;
        {
            ::std::io::_print(
                format_args!(
                    "📦 Contract deployed with initial value: {0}\n",
                    initial_value,
                ),
            );
        };
        let value_changed_filter = contract
            .ValueChanged_filter()
            .from_block(BlockNumberOrTag::Latest);
        let subscription = value_changed_filter.subscribe().await?;
        {
            ::std::io::_print(format_args!("🔔 Subscribed to ValueChanged events.\n"));
        };
        let contract_address = contract.address().clone();
        let mut stream = subscription.into_stream();
        tokio::spawn(async move {
            {
                ::std::io::_print(
                    format_args!("🚀 Listening for ValueChanged events...\n"),
                );
            };
            while let Some(result) = stream.next().await {
                match result {
                    Ok((event, log)) => {
                        if log.address() == contract_address {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "⚡\u{fe0f} Event: ValueChanged - updater: {0}, oldValue: {1}, newValue: {2}\n",
                                        event.updater,
                                        event.oldValue,
                                        event.newValue,
                                    ),
                                );
                            };
                        }
                    }
                    Err(e) => {
                        ::std::io::_print(
                            format_args!("Error processing event: {0:?}\n", e),
                        );
                    }
                }
            }
        });
        let new_value = U256::from(2);
        let tx_builder = contract.setValue(new_value).send().await?;
        let pending_tx = tx_builder.register().await?;
        let tx_hash = pending_tx.await?;
        {
            ::std::io::_print(
                format_args!(
                    "🔄 Transaction sent to set new value. Transaction hash: {0:#x}\n",
                    tx_hash,
                ),
            );
        };
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
