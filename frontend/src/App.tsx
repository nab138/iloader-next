import "./App.css";
import { client } from "./main";

function App() {
  return (
    <>
      <div>iloader web!</div>
      <button
        onClick={() => {
          client.transform("test").then((result) => {
            alert(result);
          });
        }}
      >
        test aa
      </button>
    </>
  );
}

export default App;
