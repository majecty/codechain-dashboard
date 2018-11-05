import * as _ from "lodash";
import { ChainNetworksAction } from "../actions/chainNetworks";
import { ChainNetworks } from "../requests/types";
const merge = require("deepmerge").default;
const overwriteMerge = (
  destinationArray: any,
  sourceArray: any,
  options: any
) => sourceArray;

export interface ChainNetworksState {
  chainNetworks: ChainNetworks | undefined;
}

const initialState: ChainNetworksState = {
  chainNetworks: undefined
};

export const chainNetworksReducer = (
  state = initialState,
  action: ChainNetworksAction
) => {
  switch (action.type) {
    case "SetChainNetworks": {
      const chainNetworks = action.data;
      return {
        ...state,
        chainNetworks
      };
    }
    case "UpdateChainNetworks": {
      const chainNetworks = state.chainNetworks;
      if (!chainNetworks) {
        return {
          ...state
        };
      }

      const newNodes = _.differenceBy(
        action.data.nodes,
        chainNetworks.nodes,
        "name"
      );

      const updatedNodes = _.map(chainNetworks.nodes, node => {
        const findNode = _.find(
          action.data.nodes,
          actionNode => actionNode.name === node.name
        );
        if (findNode) {
          return merge(node, findNode, { arrayMerge: overwriteMerge });
        } else {
          return node;
        }
      });

      const addedConnections =
        action.data.connectionsAdded && action.data.connectionsAdded.length > 0
          ? _.concat(chainNetworks.connections, action.data.connectionsAdded)
          : _.cloneDeep(chainNetworks.connections);

      const removedConnections =
        action.data.connectionsRemoved &&
        action.data.connectionsRemoved.length > 0
          ? _.differenceWith(
              addedConnections,
              action.data.connectionsRemoved,
              _.isEqual
            )
          : addedConnections;
      return {
        ...state,
        chainNetworks: {
          nodes: _.concat(updatedNodes, newNodes),
          connections: removedConnections
        }
      };
    }
  }
  return state;
};