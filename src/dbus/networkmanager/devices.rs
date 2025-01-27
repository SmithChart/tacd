//! # DBus interface proxies for: `org.freedesktop.NetworkManager.Device`, `org.freedesktop.NetworkManager.Device.Statistics`, `org.freedesktop.NetworkManager.Device.Wired`
//!
//! This code was generated by `zbus-xmlgen` `2.0.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/NetworkManager/Devices/2' from service 'org.freedesktop.NetworkManager' on system bus`.
//!

use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::TryFrom;
use zvariant::{OwnedValue, Type};

macro_rules! back_enum {
    ($NAME:ident {
        $( $NAMEi:ident = $KEYi:expr, )+
    }) => {
        #[allow(non_camel_case_types)]
        #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
        #[repr(u32)]
        pub enum $NAME {
        $(
            $NAMEi = $KEYi,
        )+
        }

        impl $NAME {
            pub fn from_u32(key: u32) -> Option<Self> {
                match key {
                    $(
                        $KEYi => Some($NAME::$NAMEi),
                    )+
                        _ => None,
                }
            }
            #[allow(unused)]
            pub const fn as_u32(self) -> u32 {
                self as u32
            }
        }
        impl TryFrom<OwnedValue> for $NAME {
            type Error = <u32 as TryFrom<OwnedValue>>::Error;

            fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
                // safe because AuthorityFeatures has repr u32
                Self::from_u32(<u32>::try_from(v)?)
                    .ok_or(
                        Self::Error::Message("Could not resolve enum".into())
                    )
            }
        }

    }
}

back_enum!( NMDeviceType {
    Unknown = 0,
    Generic = 14,
    Ethernet = 1,
    WIFI = 2,
    Unused1 = 3,
    Unused2 = 4,
    BT = 5,
    OLPCMesh = 6,
    Modem = 8,
    InfiniBand = 9,
    Bond = 10,
    VLAN = 11,
    ADSL = 12,
    Bridge = 13,
    Team = 15,
    TUN = 16,
    IPTunnel = 17,
    MACVLAN = 18,
    VXLAN = 19,
    VEth = 20,
});

back_enum!( NMDeviceState {
    Unknown = 0,
    Unmanaged = 10,
    Unavailable = 20,
    Disconnected = 30,
    Prepare = 40,
    Config = 50,
    NeedAuth = 60,
    IPConfig = 70,
    IPCheck = 80,
    Secondaries = 90,
    Activted = 100,
    Deactivating = 110,
    Failed = 120,
});

back_enum!( NMState {
    Unknown = 0,
    Asleep = 10,
    Disconnected = 20,
    Disconnecting = 30,
    Connecting = 40,
    ConnectedLocal = 50,
    ConnectedSite = 60,
    ConnectedGlobal = 70,
});

mod device {
    use crate::dbus::networkmanager::devices::NMDeviceState;
    use crate::dbus::networkmanager::devices::NMDeviceType;
    use zbus::dbus_proxy;

    #[dbus_proxy(
        interface = "org.freedesktop.NetworkManager.Device",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait Device {
        /// Delete method
        fn delete(&self) -> zbus::Result<()>;

        /// Disconnect method
        fn disconnect(&self) -> zbus::Result<()>;

        /// GetAppliedConnection method
        fn get_applied_connection(
            &self,
            flags: u32,
        ) -> zbus::Result<(
            std::collections::HashMap<
                String,
                std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
            >,
            u64,
        )>;

        /// Reapply method
        fn reapply(
            &self,
            connection: std::collections::HashMap<
                &str,
                std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
            >,
            version_id: u64,
            flags: u32,
        ) -> zbus::Result<()>;

        /// StateChanged signal
        #[dbus_proxy(signal)]
        fn state_changed(&self, new_state: u32, old_state: u32, reason: u32) -> zbus::Result<()>;

        /// ActiveConnection property
        #[dbus_proxy(property)]
        fn active_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// Autoconnect property
        #[dbus_proxy(property)]
        fn autoconnect(&self) -> zbus::Result<bool>;
        #[dbus_proxy(property)]
        fn set_autoconnect(&self, value: bool) -> zbus::Result<()>;

        /// AvailableConnections property
        #[dbus_proxy(property)]
        fn available_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

        /// Capabilities property
        #[dbus_proxy(property)]
        fn capabilities(&self) -> zbus::Result<u32>;

        /// DeviceType property
        #[dbus_proxy(property)]
        fn device_type(&self) -> zbus::Result<NMDeviceType>;

        /// Dhcp4Config property
        #[dbus_proxy(property)]
        fn dhcp4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// Dhcp6Config property
        #[dbus_proxy(property)]
        fn dhcp6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// Driver property
        #[dbus_proxy(property)]
        fn driver(&self) -> zbus::Result<String>;

        /// DriverVersion property
        #[dbus_proxy(property)]
        fn driver_version(&self) -> zbus::Result<String>;

        /// FirmwareMissing property
        #[dbus_proxy(property)]
        fn firmware_missing(&self) -> zbus::Result<bool>;

        /// FirmwareVersion property
        #[dbus_proxy(property)]
        fn firmware_version(&self) -> zbus::Result<String>;

        /// Interface property
        #[dbus_proxy(property)]
        fn interface(&self) -> zbus::Result<String>;

        /// Ip4Address property
        #[dbus_proxy(property)]
        fn ip4_address(&self) -> zbus::Result<u32>;

        /// Ip4Config property
        #[dbus_proxy(property)]
        fn ip4_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// Ip6Config property
        #[dbus_proxy(property)]
        fn ip6_config(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// IpInterface property
        #[dbus_proxy(property)]
        fn ip_interface(&self) -> zbus::Result<String>;

        /// LldpNeighbors property
        #[dbus_proxy(property)]
        fn lldp_neighbors(
            &self,
        ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

        /// Managed property
        #[dbus_proxy(property)]
        fn managed(&self) -> zbus::Result<bool>;
        #[dbus_proxy(property)]
        fn set_managed(&self, value: bool) -> zbus::Result<()>;

        /// Metered property
        #[dbus_proxy(property)]
        fn metered(&self) -> zbus::Result<u32>;

        /// Mtu property
        #[dbus_proxy(property)]
        fn mtu(&self) -> zbus::Result<u32>;

        /// NmPluginMissing property
        #[dbus_proxy(property)]
        fn nm_plugin_missing(&self) -> zbus::Result<bool>;

        /// PhysicalPortId property
        #[dbus_proxy(property)]
        fn physical_port_id(&self) -> zbus::Result<String>;

        /// Real property
        #[dbus_proxy(property)]
        fn real(&self) -> zbus::Result<bool>;

        /// State property
        #[dbus_proxy(name = "State", property)]
        fn get_state(&self) -> zbus::Result<NMDeviceState>;

        /// StateReason property
        #[dbus_proxy(property)]
        fn state_reason(&self) -> zbus::Result<(u32, u32)>;

        /// Udi property
        #[dbus_proxy(property)]
        fn udi(&self) -> zbus::Result<String>;
    }
}

mod statistics {
    use zbus::dbus_proxy;
    #[dbus_proxy(
        interface = "org.freedesktop.NetworkManager.Device.Statistics",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait Statistics {
        /// PropertiesChanged signal
        #[dbus_proxy(signal)]
        fn properties_changed(
            &self,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;

        /// RefreshRateMs property
        #[dbus_proxy(property)]
        fn refresh_rate_ms(&self) -> zbus::Result<u32>;
        #[dbus_proxy(property)]
        fn set_refresh_rate_ms(&self, value: u32) -> zbus::Result<()>;

        /// RxBytes property
        #[dbus_proxy(property)]
        fn rx_bytes(&self) -> zbus::Result<u64>;

        /// TxBytes property
        #[dbus_proxy(property)]
        fn tx_bytes(&self) -> zbus::Result<u64>;
    }
}

mod wired {
    use zbus::dbus_proxy;
    #[dbus_proxy(
        interface = "org.freedesktop.NetworkManager.Device.Wired",
        default_service = "org.freedesktop.NetworkManager"
    )]
    trait Wired {
        /// PropertiesChanged signal
        #[dbus_proxy(signal)]
        fn properties_changed(
            &self,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;

        /// Carrier property
        #[dbus_proxy(property)]
        fn carrier(&self) -> zbus::Result<bool>;

        /// HwAddress property
        #[dbus_proxy(property)]
        fn hw_address(&self) -> zbus::Result<String>;

        /// PermHwAddress property
        #[dbus_proxy(property)]
        fn perm_hw_address(&self) -> zbus::Result<String>;

        /// S390Subchannels property
        #[dbus_proxy(property)]
        fn s390subchannels(&self) -> zbus::Result<Vec<String>>;

        /// Speed property
        #[dbus_proxy(property)]
        fn speed(&self) -> zbus::Result<u32>;
    }
}

pub mod ip4 {
    use zbus::dbus_proxy;
    use zvariant::OwnedValue;
    use zvariant_derive::{DeserializeDict, SerializeDict, Type};

    #[derive(DeserializeDict, SerializeDict, Type, Debug, OwnedValue)]
    pub struct IP4Adress {
        prefix: u32,
        address: String,
    }

    #[dbus_proxy(
        interface = "org.freedesktop.NetworkManager.IP4Config",
        default_service = "org.freedesktop.NetworkManager"
    )]
    pub trait IP4Config {
        /// PropertiesChanged signal
        #[dbus_proxy(signal)]
        fn properties_changed(
            &self,
            properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        ) -> zbus::Result<()>;

        /// AddressData property
        #[dbus_proxy(name = "AddressData", property)]
        fn address_data2(
            &self,
        ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

        // TODO: This is currently not working: https://gitlab.freedesktop.org/dbus/zbus/-/issues/241
        /// AddressData property
        #[dbus_proxy(property)]
        fn address_data(
            &self,
            //) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;
        ) -> zbus::Result<IP4Adress>;

        /// Addresses property
        #[dbus_proxy(property)]
        fn addresses(&self) -> zbus::Result<Vec<Vec<u32>>>;

        /// DnsOptions property
        #[dbus_proxy(property)]
        fn dns_options(&self) -> zbus::Result<Vec<String>>;

        /// DnsPriority property
        #[dbus_proxy(property)]
        fn dns_priority(&self) -> zbus::Result<i32>;

        /// Domains property
        #[dbus_proxy(property)]
        fn domains(&self) -> zbus::Result<Vec<String>>;

        /// Gateway property
        #[dbus_proxy(property)]
        fn gateway(&self) -> zbus::Result<String>;

        /// NameserverData property
        #[dbus_proxy(property)]
        fn nameserver_data(
            &self,
        ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

        /// Nameservers property
        #[dbus_proxy(property)]
        fn nameservers(&self) -> zbus::Result<Vec<u32>>;

        /// RouteData property
        #[dbus_proxy(property)]
        fn route_data(
            &self,
        ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

        /// Routes property
        #[dbus_proxy(property)]
        fn routes(&self) -> zbus::Result<Vec<Vec<u32>>>;

        /// Searches property
        #[dbus_proxy(property)]
        fn searches(&self) -> zbus::Result<Vec<String>>;

        /// WinsServerData property
        #[dbus_proxy(property)]
        fn wins_server_data(&self) -> zbus::Result<Vec<String>>;

        /// WinsServers property
        #[dbus_proxy(property)]
        fn wins_servers(&self) -> zbus::Result<Vec<u32>>;
    }
}
pub use device::DeviceProxy;
pub use statistics::StatisticsProxy;
pub use wired::WiredProxy;
