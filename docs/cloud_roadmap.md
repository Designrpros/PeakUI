# PeakCloud & Relay Roadmap

PeakCloud is the P2P networking and discovery layer that enables the Peak Swarm to communicate securely across local and global networks.

## Phase 1: Local Discovery (Active 🚧)
*Focus: Fast, zero-config LAN synchronization.*

- [x] **mDNS Discovery**: Automatic device finding on the local network.
- [x] **Identity Handshake**: Secure certificate exchange between local agents.
- [x] **Local Mirroring**: Logic for detecting neighbors and initiating data replication.
- [x] **AES-256 Mesh**: Fully encrypted communication within the local network.

---

## Phase 2: P2P Relay & Mesh (Future 🔮)
*Focus: Seamless global connectivity without open ports.*

- [ ] **Hole Punching**: libp2p/WebRTC-based NAT traversal for direct device links.
- [ ] **Relay Fallbacks**: TURN/STUN server support for complex network topologies.
- [ ] **Decentralized Relay**: Leveraging other Peak devices as secure relays.
- [ ] **Sovereign Pairing**: Direct E2EE pairing via hardware tokens or QR codes.

---

## Phase 3: Intelligence Swarm (Future 🔮)
*Focus: Distributed agent coordination and intent relay.*

- [ ] **Remote Intents**: Trigger actions on other devices (e.g., "Run build on my x86 server").
- [ ] **Swarm Coordination**: Multi-agent task delegation across the mesh.
- [ ] **Distributed Index**: Real-time sync of the global semantic index.
- [ ] **Asset Mirroring**: Bandwidth-optimized transfer of large neural models.

---

## Phase 4: Global PeakNet (Long-term 🌌)
*Focus: Decentralized infrastructure and universal identity.*

- [ ] **PeakNet Domain**: Decentralized naming system for Peak devices.
- [ ] **Infinite Redundancy**: Automatic data sharding across all authorized swarm nodes.
- [ ] **Global Identity**: Roaming profiles that follow you across the mesh.
- [ ] **Sovereign Mesh**: A fully self-healing, global network of Peak agents.
