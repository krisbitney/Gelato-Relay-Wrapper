# Gelato Relay Wrap

Enables easy and streamlined interactions with the Gelato Network Relayer. It offers functionalities such as making relay calls, fetching transaction statuses, and estimating fees, exposed to WASM using Polywrap. Inspired in [Gelato Relay SDK](https://github.com/gelatodigital/relay-sdk)

## Methods

### `callWithSyncFee(request, options)`

Initiates a relayed call with a synchronous fee.

- `request`: Contains the `chainId`, `target`, `data`, `feeToken`, and `isRelayContext` fields.
- `options`: (Optional) Configuration for the relay. Includes `gasLimit` and `retries`.

**Returns:** A `RelayResponse` containing the `taskId`.

### `sponsoredCall(request, sponsorApiKey, options)`

Initiates a sponsored relay call. This method is beneficial when you have a sponsor covering the relay fee.

- `request`: Contains the `chainId`, `target`, and `data` fields.
- `sponsorApiKey`: The API key for the sponsor.
- `options`: (Optional) Configuration for the relay. Includes `gasLimit` and `retries`.

**Returns:** A `RelayResponse` containing the `taskId`.

### `getEstimatedFee(chainId, paymentToken, gasLimit, isHighPriority, gasLimitL1)`

Estimates the fee for a relay call.

- `chainId`: The blockchain chain ID.
- `paymentToken`: Token used for payment.
- `gasLimit`: Gas limit for the transaction.
- `isHighPriority`: Boolean indicating the priority of the transaction.
- `gasLimitL1`: (Optional) Gas limit for Layer 1.

**Returns:** The estimated fee as a `BigInt`.

### `getTaskStatus(taskId)`

Fetches the status of a given task using its ID.

- `taskId`: The ID of the task you want to check the status of.

**Returns:** A `TransactionStatusResponse` object containing details about the task's status.

## Data Types

- `RelayResponse`: Contains a `taskId` indicating the ID of the relayed task.
- `TransactionStatusResponse`: Contains details about the status of a given task, including the `chainId`, `taskId`, `taskState`, and various date and transaction details.
- `TaskState`: An enum representing the possible states a task can be in.
- `SponsoredCallRequest`: Contains details required to initiate a sponsored call.
- `RelayRequestOptions`: Contains optional configurations for relay calls.
- `CallWithSyncFeeRequest`: Contains details required to initiate a call with a synchronous fee.
