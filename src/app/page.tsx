import Head from "next/head";

export default function Home() {
  return (
    <>
      <Head>
        <title>QMeet Control</title>
        <meta name="description" content="QMeet Application Control" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          height: "100vh",
          fontFamily: "sans-serif",
          background: "#fafafa",
        }}
      >
        <h1
          style={{
            fontSize: "2rem",
            marginBottom: "1rem",
            color: "#333",
          }}
        >
          QMeet Control
        </h1>
        <div style={{ display: "flex", gap: "1rem" }}>
          <button
            style={{
              padding: "0.75rem 1.5rem",
              background: "#0070f3",
              color: "#fff",
              border: "none",
              borderRadius: "0.5rem",
              cursor: "pointer",
            }}
          >
            Modify Shortcut
          </button>
          <button
            style={{
              padding: "0.75rem 1.5rem",
              background: "#ff4c4c",
              color: "#fff",
              border: "none",
              borderRadius: "0.5rem",
              cursor: "pointer",
            }}
          >
            Quit App
          </button>
        </div>
      </main>
    </>
  );
}
