[build]
rustflags = ["-C", "link-args=-s"]
CONTRACT_NAME = process.env.CONTRACT_NAME || 'crowdfund.srijuu.testnet'