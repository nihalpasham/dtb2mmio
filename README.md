# dtb2mmio 
dtb2mmio is a tiny command line utility that can parse device-tree blobs or flattened device-trees. You can quickly retrieve a peripheral's base-address or address-range. 

dtb's are used by many large projects such as android, linux and openwrt and as such are an authoritative source of information. So, rather than scour the internet for good documentation, we can quickly consult a dtb for info. But `dts` source files can be pretty scattered (and numerous) at times. `dtb2mmio` can help - it is a nice quality of life improvement, 

- saves time and 
- makes it easier in the absence of documentation (i.e. reference manuals or data-sheets), 

![dtb2mmio](https://user-images.githubusercontent.com/20253082/215260982-d1f65920-f371-466e-964b-dea627f24ac3.png)

parsing device trees with rust - [talk](https://www.youtube.com/live/xJ99jrxwbTk?feature=share): 

## Usage:

- Clone the repo and run the following command to quickly parse a dtb for the `ethernet` peripheral. 

```sh
cargo run --release -- imx8mn-ddr4-evk.dtb ethernet
   Compiling dtb2rust v0.1.0 (/Users/nihal.pasham/devspace/rust/projects/exp/dtb2rust)
    Finished release [optimized] target(s) in 0.31s
     Running `target/release/dtb2rust imx8mn-ddr4-evk.dtb`
node-depth: 4, /soc@0/bus@30800000/ethernet@30be0000/: <0x30be0000, 0x10000>
node-depth: 4, /soc@0/bus@30800000/ethernet@30be0000/: <0x0>
node-depth: 5, /soc@0/bus@30800000/ethernet@30be0000/mdio/: <0x0>
node-depth: 6, /soc@0/bus@30800000/ethernet@30be0000/mdio/ethernet-phy@0/: <0x0>
```
- As you can see below, this dtb does not include a `uart` peripheral but this is (actually) a bit misleading - the `imx8mn-nano SoC` includes 4 uarts except that they're referred to as `serial peripherals`. So, you'll have try a few `names` if you're first try comes up empty.

```sh
cargo run --release -- imx8mn-ddr4-evk.dtb uart
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/dtb2mmio imx8mn-ddr4-evk.dtb uart`

cargo run --release -- imx8mn-ddr4-evk.dtb serial
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/dtb2mmio imx8mn-ddr4-evk.dtb serial`
node-depth: 5, /soc@0/bus@30800000/spba-bus@30800000/serial@30860000/: <0x30860000, 0x10000>
node-depth: 5, /soc@0/bus@30800000/spba-bus@30800000/serial@30880000/: <0x30880000, 0x10000>
node-depth: 5, /soc@0/bus@30800000/spba-bus@30800000/serial@30890000/: <0x30890000, 0x10000>
node-depth: 4, /soc@0/bus@30800000/serial@30a60000/: <0x30a60000, 0x10000>
```

### Limitations:

- only supports nodes with upto 500 properties.
- as dtb's dont have a standard naming convention, you'll have to try a few permutations and combinations, if you cant find your peripheral. 