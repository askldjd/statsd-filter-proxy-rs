const udp = require('dgram');
const server = udp.createSocket('udp4');
const client = udp.createSocket('udp4');

const config = {
  listenPort: 8125,
  forward: {
    host: '127.0.0.1',
    port: 8126,
  },
  metricBlocklist: [
    "foo"
  ]
}

function blacklistMetric(metric) {
  for (const substring of config.metricBlocklist) {
    if (metric.includes(substring)) {
        return true;
    }
  }
  return false;
}

server.on('message', (msg) => {
  if (blacklistMetric(msg)) {
    return;
  }

  client.send(msg, config.forward.port, config.forward.host, (error) => {
    if (error) {
      console.log(`Unable to forward datagram to ${config.forward}, ${error}`);
      process.exit(-1);
    }
  });
});

server.on('listening', () => {
  console.log(`Listening at ${server.address().address}:${server.address().port}`);
});

server.on('close', () => {
  console.log('UDP server socket is closed');
});

server.on('error', (error) => {
  console.warn(`UDP server Error: ${error}`);
  server.close();
});

server.bind(config.listenPort);
