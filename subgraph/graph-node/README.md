The Graph Node local environment spins up a Graph Node instance, an IPFS node and a Postgres database instance through a Docker Compose specification. In order for the Graph Node to access the blockchain data, You should provide a Substreams API token, which is automatically fetched from the `SUBSTREAMS_API_TOKEN` environment variable of your system.

The entrypoint to set up the Graph Node local environment is the `start.sh` script, which executes everything necessary in your computer. 
```bash
./start.sh
```