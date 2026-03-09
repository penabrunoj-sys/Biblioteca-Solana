// No imports needed: web3, anchor, pg and more are globally available

describe("biblioteca", () => {

  it("crear biblioteca", async () => {

    const nombreBiblioteca = "Biblioteca Digital";

    const txHash = await pg.program.methods
      .crearBiblioteca(nombreBiblioteca)
      .accounts({
        owner: pg.wallet.publicKey,
      })
      .rpc();

    console.log(`Tx crear biblioteca: ${txHash}`);

    await pg.connection.confirmTransaction(txHash);

  });

  it("agregar libro", async () => {

    const nombreLibro = "Rust para Solana";
    const paginas = 120;
    const precio = new BN(1000000);

    const txHash = await pg.program.methods
      .agregarLibro(nombreLibro, paginas, precio)
      .accounts({
        owner: pg.wallet.publicKey,
      })
      .rpc();

    console.log(`Tx agregar libro: ${txHash}`);

    await pg.connection.confirmTransaction(txHash);

  });

  it("ver libros", async () => {

    const txHash = await pg.program.methods
      .verLibros()
      .accounts({
        owner: pg.wallet.publicKey,
      })
      .rpc();

    console.log(`Tx ver libros: ${txHash}`);

    await pg.connection.confirmTransaction(txHash);

  });

});
