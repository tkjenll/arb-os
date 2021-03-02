/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import {
  ethers,
  EventFilter,
  Signer,
  BigNumber,
  BigNumberish,
  PopulatedTransaction,
} from "ethers";
import {
  Contract,
  ContractTransaction,
  Overrides,
  CallOverrides,
} from "@ethersproject/contracts";
import { BytesLike } from "@ethersproject/bytes";
import { Listener, Provider } from "@ethersproject/providers";
import { FunctionFragment, EventFragment, Result } from "@ethersproject/abi";
import { TypedEventFilter, TypedEvent, TypedListener } from "./commons";

interface ArbAddressTableInterface extends ethers.utils.Interface {
  functions: {
    "addressExists(address)": FunctionFragment;
    "compress(address)": FunctionFragment;
    "decompress(bytes,uint256)": FunctionFragment;
    "lookup(address)": FunctionFragment;
    "lookupIndex(uint256)": FunctionFragment;
    "register(address)": FunctionFragment;
    "size()": FunctionFragment;
  };

  encodeFunctionData(
    functionFragment: "addressExists",
    values: [string]
  ): string;
  encodeFunctionData(functionFragment: "compress", values: [string]): string;
  encodeFunctionData(
    functionFragment: "decompress",
    values: [BytesLike, BigNumberish]
  ): string;
  encodeFunctionData(functionFragment: "lookup", values: [string]): string;
  encodeFunctionData(
    functionFragment: "lookupIndex",
    values: [BigNumberish]
  ): string;
  encodeFunctionData(functionFragment: "register", values: [string]): string;
  encodeFunctionData(functionFragment: "size", values?: undefined): string;

  decodeFunctionResult(
    functionFragment: "addressExists",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "compress", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "decompress", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "lookup", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "lookupIndex",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "register", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "size", data: BytesLike): Result;

  events: {};
}

export class ArbAddressTable extends Contract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  listeners<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter?: TypedEventFilter<EventArgsArray, EventArgsObject>
  ): Array<TypedListener<EventArgsArray, EventArgsObject>>;
  off<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  on<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  once<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  removeListener<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  removeAllListeners<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>
  ): this;

  listeners(eventName?: string): Array<Listener>;
  off(eventName: string, listener: Listener): this;
  on(eventName: string, listener: Listener): this;
  once(eventName: string, listener: Listener): this;
  removeListener(eventName: string, listener: Listener): this;
  removeAllListeners(eventName?: string): this;

  queryFilter<EventArgsArray extends Array<any>, EventArgsObject>(
    event: TypedEventFilter<EventArgsArray, EventArgsObject>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEvent<EventArgsArray & EventArgsObject>>>;

  interface: ArbAddressTableInterface;

  functions: {
    addressExists(addr: string, overrides?: CallOverrides): Promise<[boolean]>;

    "addressExists(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<[boolean]>;

    compress(addr: string, overrides?: Overrides): Promise<ContractTransaction>;

    "compress(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<ContractTransaction>;

    decompress(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string, BigNumber]>;

    "decompress(bytes,uint256)"(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string, BigNumber]>;

    lookup(addr: string, overrides?: CallOverrides): Promise<[BigNumber]>;

    "lookup(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<[BigNumber]>;

    lookupIndex(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string]>;

    "lookupIndex(uint256)"(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string]>;

    register(addr: string, overrides?: Overrides): Promise<ContractTransaction>;

    "register(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<ContractTransaction>;

    size(overrides?: CallOverrides): Promise<[BigNumber]>;

    "size()"(overrides?: CallOverrides): Promise<[BigNumber]>;
  };

  addressExists(addr: string, overrides?: CallOverrides): Promise<boolean>;

  "addressExists(address)"(
    addr: string,
    overrides?: CallOverrides
  ): Promise<boolean>;

  compress(addr: string, overrides?: Overrides): Promise<ContractTransaction>;

  "compress(address)"(
    addr: string,
    overrides?: Overrides
  ): Promise<ContractTransaction>;

  decompress(
    buf: BytesLike,
    offset: BigNumberish,
    overrides?: CallOverrides
  ): Promise<[string, BigNumber]>;

  "decompress(bytes,uint256)"(
    buf: BytesLike,
    offset: BigNumberish,
    overrides?: CallOverrides
  ): Promise<[string, BigNumber]>;

  lookup(addr: string, overrides?: CallOverrides): Promise<BigNumber>;

  "lookup(address)"(
    addr: string,
    overrides?: CallOverrides
  ): Promise<BigNumber>;

  lookupIndex(index: BigNumberish, overrides?: CallOverrides): Promise<string>;

  "lookupIndex(uint256)"(
    index: BigNumberish,
    overrides?: CallOverrides
  ): Promise<string>;

  register(addr: string, overrides?: Overrides): Promise<ContractTransaction>;

  "register(address)"(
    addr: string,
    overrides?: Overrides
  ): Promise<ContractTransaction>;

  size(overrides?: CallOverrides): Promise<BigNumber>;

  "size()"(overrides?: CallOverrides): Promise<BigNumber>;

  callStatic: {
    addressExists(addr: string, overrides?: CallOverrides): Promise<boolean>;

    "addressExists(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<boolean>;

    compress(addr: string, overrides?: CallOverrides): Promise<string>;

    "compress(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<string>;

    decompress(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string, BigNumber]>;

    "decompress(bytes,uint256)"(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<[string, BigNumber]>;

    lookup(addr: string, overrides?: CallOverrides): Promise<BigNumber>;

    "lookup(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    lookupIndex(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<string>;

    "lookupIndex(uint256)"(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<string>;

    register(addr: string, overrides?: CallOverrides): Promise<BigNumber>;

    "register(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    size(overrides?: CallOverrides): Promise<BigNumber>;

    "size()"(overrides?: CallOverrides): Promise<BigNumber>;
  };

  filters: {};

  estimateGas: {
    addressExists(addr: string, overrides?: CallOverrides): Promise<BigNumber>;

    "addressExists(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    compress(addr: string, overrides?: Overrides): Promise<BigNumber>;

    "compress(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<BigNumber>;

    decompress(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    "decompress(bytes,uint256)"(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    lookup(addr: string, overrides?: CallOverrides): Promise<BigNumber>;

    "lookup(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    lookupIndex(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    "lookupIndex(uint256)"(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    register(addr: string, overrides?: Overrides): Promise<BigNumber>;

    "register(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<BigNumber>;

    size(overrides?: CallOverrides): Promise<BigNumber>;

    "size()"(overrides?: CallOverrides): Promise<BigNumber>;
  };

  populateTransaction: {
    addressExists(
      addr: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    "addressExists(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    compress(
      addr: string,
      overrides?: Overrides
    ): Promise<PopulatedTransaction>;

    "compress(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<PopulatedTransaction>;

    decompress(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    "decompress(bytes,uint256)"(
      buf: BytesLike,
      offset: BigNumberish,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    lookup(
      addr: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    "lookup(address)"(
      addr: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    lookupIndex(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    "lookupIndex(uint256)"(
      index: BigNumberish,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    register(
      addr: string,
      overrides?: Overrides
    ): Promise<PopulatedTransaction>;

    "register(address)"(
      addr: string,
      overrides?: Overrides
    ): Promise<PopulatedTransaction>;

    size(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    "size()"(overrides?: CallOverrides): Promise<PopulatedTransaction>;
  };
}
