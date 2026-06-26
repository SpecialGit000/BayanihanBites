# BayanihanBites

Decentralized local food delivery escrow system protecting community drivers and small kitchens in SEA.

### Problem & Solution
Juan, an independent delivery rider in Bulacan, faces high cash-on-delivery (COD) cancellation risks where customers fake orders, forcing him to pay the restaurant out of pocket and lose his day's income. BayanihanBites uses Soroban smart contracts to hold the customer's food payment and delivery fee in escrow, releasing the food payment to the kitchen when Juan picks up the order, and releasing the delivery fee to Juan when the customer signs for the delivery.

### Timeline
* **Day 1-2:** Soroban escrow multi-party routing contract architecture layout.
* **Day 3-4:** Mobile web integration with basic geolocation order claim UX components.
* **Day 5:** Live Stellar Testnet sandbox demo tracking milestone distribution states.

### Stellar Features Used
* **Soroban Smart Contracts:** Manages atomic multi-step order distribution payouts safely without middleman fees.
* **Stablecoin Integration:** Ensures cost predictability for gig workers navigating tight daily margin limits.
* **Low Transaction Overhead:** Executing state updates costs fractions of a cent, outcompeting traditional Web2 logistics networks.

### Vision & Purpose
To eliminate the economic vulnerability inherent to informal cash-on-delivery markets, building real-time peer-to-peer security frameworks for unbanked micro-entrepreneurs.

### Prerequisites
* Rust `v1.74.0+`
* Soroban CLI `v20.0.0+`
* Target: `wasm32-unknown-unknown`

### How to Build
```bash
soroban contract build