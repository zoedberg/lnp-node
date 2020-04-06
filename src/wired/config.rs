// Lightning network protocol (LNP) daemon suit
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use std::{fmt, str::FromStr};
use clap::Clap;

use lnpbp::common::internet::{InetSocketAddr, InetAddr};

use super::{wire, bus};
use lnpbp::miniscript::bitcoin::hashes::core::fmt::{Formatter, Error};

#[derive(Clap)]
#[clap(
    version = "0.0.1",
    author = "Dr Maxim Orlovsky <orlovsky@pandoracore.com>",
    about =  "LNP wired: Lightning wire P2P daemon; part of Lightning network protocol suite"
)]
pub struct Opts {
    #[clap(short = "c", long = "config", default_value = "wired.toml")]
    pub config: String,

    #[clap(short = "v", long = "verbose", min_values=0, max_values=4, parse(from_occurrences))]
    pub verbose: i32,

    /// IPv4, IPv6 or Tor address to listen for incoming connections from LN peers
    #[clap(short = "i", long = "inet-addr", default_value = "0.0.0.0", env="LNP_WIRED_INET_ADDR",
           parse(try_from_str = InetAddr::from_str))]
    address: InetAddr,

    /// Use custom port to listen for incoming connections from LN peers
    #[clap(short = "p", long = "port", default_value = "9735", env="LNP_WIRED_PORT")]
    port: u16,
}

// TODO: Unlock this feature (specifying connection points at start) lately
/*
(@arg connect: -c --connect ... "Nodes to connect at after the launch \
    (in form of <node_id>@<inet_addr>[:<port>], \
    where <inet_addr> can be IPv4, IPv6 or TORv3 internet address)")
    */


#[derive(Clone, PartialEq, Eq, Debug, Display)]
#[display_from(Debug)]
pub struct Config {
    pub verbose: u8,
    pub lnp2p_addr: InetSocketAddr,
    pub subscribe_addr: String,
}

impl From<Opts> for Config {
    fn from(opts: Opts) -> Self {
        Self {
            verbose: opts.verbose as u8,
            lnp2p_addr: InetSocketAddr::tcp(opts.address, opts.port),
            subscribe_addr: "".to_string(),
        }
    }
}