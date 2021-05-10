// This is the original implementation of statsd-filter-proxy. It is a very
// tiny nodejs program with decent performance characteristics. This version
// is used as the performance baseline. If the Rust version is slower than 
// Nodejs, then we are probably doing it wrong.

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
    "foo1",
    "foo2"
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

server.on('message', (pkt) => {
  for (const msg of pkt.toString().split("\n")) {  
    if (blacklistMetric(msg)) {
      continue;
    }

    client.send(msg, config.forward.port, config.forward.host, (error) => {
      if (error) {
        console.log(`Unable to forward datagram to ${config.forward}, ${error}`);
        process.exit(-1);
      }
    });
  }
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
