const endpoint = {
    http: {
      devnet: 'http://api.devnet.solana.com',
      testnet: 'http://api.testnet.solana.com',
      'mainnet-beta': 'http://api.mainnet-beta.solana.com/',
    },
    https: {
      devnet: 'https://api.devnet.solana.com',
      testnet: 'https://api.testnet.solana.com',
      'mainnet-beta': 'https://api.mainnet-beta.solana.com/',
    },
  };
  
  const availableClusters = ['devnet', 'testnet', 'mainnet-beta'];
  
  function clusterApiUrl(cluster = 'devnet', tls = true) {
    const key = tls ? 'https' : 'http';
  
    if (!availableClusters.includes(cluster)) {
      throw new Error(`Unknown cluster: ${cluster}`);
    }
  
    return endpoint[key][cluster];
  }
  
  module.exports = { clusterApiUrl };
  