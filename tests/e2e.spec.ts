import {PolywrapClient} from "@polywrap/client-js";
import * as Types from "./types/wrap";
import * as path from "path";


jest.setTimeout(360000);

describe("Gelato Relay Wrapper", () => {
  const client = new PolywrapClient()
  let uri: string = `wrap://fs/${path.join(__dirname, "../")}/build`;

  test("callWithSyncFee should return a valid response", async () => {
    // target contract address
    const myDummyWallet = "0xA045eb75e78f4988d42c3cd201365bDD5D76D406";

    // using a human-readable ABI for generating the payload
    const abi = "function sendToFriend(address _token,address _to,uint256 _amount)";

    // sendToFriend arguments
    const feeToken = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
    const vitalik = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
    const amountToSend = 1_000_000_000_000_000;

    // encode the payload
    const data = await client.invoke<string>({
      uri: "wrap://ens/wraps.eth:ethereum@1.1.0",
      method: "encodeFunction",
      args: {
        method: abi,
        args: [feeToken, vitalik, amountToSend.toString()]
      }
    })
    if (!data.ok) throw data.error;

    const result = await client.invoke<Types.RelayResponse>({
      uri,
      method: "callWithSyncFee",
      args: {
        request: {
          chainId: "5", // goerli
          target: myDummyWallet,
          data: Buffer.from(data.value, "hex"),
          feeToken,
          isRelayContext: true,
        },
        options: {
          retries: 2,
        }
      }
    });

    if (!result.ok) throw result.error
    expect(result.value.taskId).toBeDefined();
    console.log(result.value)
  });

  test("sponsoredCall should return a valid response", async () => {
    // set up target address and function signature abi
    const counter = "0xEEeBe2F778AA186e88dCf2FEb8f8231565769C27";
    const abi = "function increment()";
    const signer = "0x";

    // encode the payload
    const data = await client.invoke<string>({
      uri: "wrap://ens/wraps.eth:ethereum@1.1.0",
      method: "encodeFunction",
      args: {
        method: abi,
        args: [counter, abi, signer]
      }
    })
    if (!data.ok) throw data.error;

    const result = await client.invoke<Types.RelayResponse>({
      uri,
      method: "sponsoredCall",
      args: {
        request: {
          chainId: "5", // goerli
          target: counter,
          data,
        },
        options: {
          gasLimit: "1000000",
          retries: 2,
        }
      }
    });

    if (!result.ok) throw result.error
    expect(result.value.taskId).toBeDefined();
  });

  test("getEstimatedFee should return a valid response", async () => {
    const result = await client.invoke<Types.BigInt>({
      uri,
      method: "getEstimatedFee",
      args: {
        chainId: "5",
        paymentToken: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",
        gasLimit: "1000000",
        isHighPriority: true,
        gasLimitL1: "1000000",
      }
    });
    if (!result.ok) throw result.error
  });

  test("getTaskStatus should return a valid response", async () => {
    const result = await client.invoke<Types.TransactionStatusResponse>({
      uri,
      method: "getTaskStatus",
      args: { taskId: "0x93a3defc618ff97c32a37bdd567b15c50748a5c3e8e858bca67f0c967b74a7fe" }
    });
    if (!result.ok) throw result.error

    const expected = {
      chainId: 5,
      taskId: "0x93a3defc618ff97c32a37bdd567b15c50748a5c3e8e858bca67f0c967b74a7fe",
      taskState: "ExecSuccess",
      creationDate: "2022-10-10T10:15:03.932Z",
      executionDate: "2022-10-10T10:15:28.718Z",
      transactionHash: "0x9d260d1bbe075be0cda52a3271df062748f3182ede91b3aae5cd115f7b26552b",
      blockNumber: 7744557
    };
    expect(result.value).toStrictEqual(expected);
  });
})

