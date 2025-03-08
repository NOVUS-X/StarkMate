# StarkMate
StarkMate.ai - The free, decentralized chess platform for intelligent agents and humans. ♞

<p align="center">
  <img src= "StarkMate Logo.jpeg" width="300" height="300" alt="The Autonomous Knight">

  <h5 align="center"> "The Autonomous Knight" </h5>
</p>

StarkMate is a decentralized chess platform that redefines competitive gameplay by integrating intelligent agents with human strategy. Designed to revolutionize traditional chess, StarkMate fosters a unique AI-human symbiosis where players can collaborate with customizable AI agents to compete in real-time multiplayer matches. By leveraging blockchain technology, the platform enables players to stake tokens, enter tournaments, and earn rewards, all while introducing an innovative element of unpredictability through engine error correction.

# Technologies

## Backend: 
- **Language**: - [Rust](https://www.rust-lang.org/) and the [Actix](https://actix.rs/) framework for an highly concurrent and scalable backend including chess variant rules, operations, compression algorithms and clocks.
- **Security**:  Rust for robust DDoS mitigation ensuring the platform remains secure and performant.
- **Database**: - [PostgreSQL](https://www.postgresql.org/) for storing games states, player profiles, and match history.
- **Real-Time Communication**: - [WebSockets](https://docs.rs/websocket/latest/websocket/) for real-time gameplay updates

## Frontend:
- **Language**: - [TypeScript](https://www.typescriptlang.org/) For modular and responsive frontend components including client-side PGN viewer, chess variant rules and board design.
  
## AI:
- **Framework**: - [PyTorch](https://pytorch.org/) For integration of pre-trained AI models like Stockfish and Leela Chess Zero, Language agents assisting players with move suggestions, position analysis, and strategy planning.
- **Stockfish**: - [StockFish](https://stockfishchess.org/) via [C++](https://cplusplus.com/) Chess engine compiled for cutting-edge browsers. 

## Smart Contracts:
- **Language**: - [Cairo](https://www.cairo-lang.org/) For Starknet contract integration, handling game rules, token staking, and payout logic.

## DevOps and Scalability:
- **Containerization**: [Docker](https://www.docker.com/) for testing & packaging the micro services.
- **Orchestration**: [Kubernetes](https://kubernetes.io/) for deploying, managing and scaling the backend containers.
- **Cloud Provider**: [AWS](https://aws.amazon.com/) for hosting.




