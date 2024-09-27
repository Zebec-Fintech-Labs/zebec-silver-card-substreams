# Substreams-powered Subgraph 

## Develop, Build and Deploy

You must have authenticated and build the substreams project before working on the subgraph as it is 
powered by substreams pkg. Follow the [usage](/README.md) documentation if you have not.

Make sure that you have a graph-node running locally (or set LOCAL_GRAPH_NODE_HOSTNAME)
If you are inside the `devcontainer` then you can directly skip to [building subgraph](/subgraph/README.md#building-subgraph)

### Running graph-node without devcontainer
If you are not using `devcontainer`, You can run graph-node by running `start.sh` located in graph-node folder in 
subgraph folder. But before that you must export substreams api token value to environment variable `SUBSTREAMS_API_TOKEN`.
You can do so in following way:

```bash
# assuming you are in subgraph directory
source ./../.substreams.env 
```

Then run `start.sh` file in graph-node folder

```bash
# Assuming you in subgraph directory
./graph-node/start.sh
```
This will spin up graph-node docker image along with postges and ipfs images.

### Building subgraph

First, install the project dependencies:

```bash
npm install
```

Run the following to generate to corresponding protobuf types in assembly scripts. Skip this step if they are already generated.

```bash
npm run generate
```

Run the following to build the subgrahp for deployment
```
npm run build
```

### Deploying to local graph

Before deploying run this to remove the locally deployed subgraph in graph node. Skip this if you haven't deployed yet.

```bash
# rince and repeat
npm run remove-local
```

Run the following to deploy the subgraph in the graph-node that is running in docker service.

```bash
npm run deploy-local
```

### Deploying to subgraph studio

If you want to deploy the subgraph to subgraph studio, then run the following command.

```bash
npm run deploy-studio
```

### Publishing subgraph

Run the following to publish the subgrah

```bash
npm run publish
```