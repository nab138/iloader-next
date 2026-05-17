import Header from "./parts/Header";
import Devices from "./parts/Devices";
import Account from "./parts/Account";

function App() {
  return (
    <>
      <Header />
      <main className="flex gap-5 m-2 mt-3 flex-wrap">
        <Account />
        <Devices />
      </main>
    </>
  );
}

export default App;
