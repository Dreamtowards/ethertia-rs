use std::net::Ipv4Addr;
use enet::*;

fn client()
{
    let enet = Enet::new().unwrap();

    let mut host = enet.create_host(
        None,
        10,
        ChannelLimit::Maximum,
        BandwidthLimit::Unlimited,
        BandwidthLimit::Unlimited).unwrap();

    host.connect(&Address::new(Ipv4Addr::LOCALHOST, 9001), 10, 0);


}

fn server()
{

}