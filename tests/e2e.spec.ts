import {PolywrapClient} from "@polywrap/client-js";
import * as Types from "./types/wrap";
import {Commands} from "@polywrap/cli-js";
import * as path from "path";


jest.setTimeout(360000);

describe("Gelato Relay Wrapper", () => {
  const client = new PolywrapClient()
  const uri: string = "wrap://fs/../../build";

  beforeAll(async () => {
    await Commands.build(undefined, { cwd: path.join(__dirname, "../..") });
  });

  test("callWithSyncFee should return a valid response", async () => {
    // const request = {
    //     chainId: BigInt!
    //     target: String!
    //     data: Bytes!
    //     feeToken: String!
    //     isRelayContext: Boolean
    //   };
    // const options = {
    //   gasLimit: BigInt
    //   retries: Int
    // };
    // const result = client.invoke<Types.RelayResponse>({
    //   uri,
    //   method: "callWithSyncFee",
    //   args: { request, options }
    // });
  });

  test("sponsoredCall should return a valid response", async () => {

  });

  test("getEstimatedFee should return a valid response", async () => {

  });

  test("getTaskStatus should return a valid response", async () => {

  });
})

