import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import Board from "./components/Board";
import GameMenu from "./components/GameMenu";
import { ToastContainer, toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

function App() {
  const [view, setView] = useState([]);
  const [isFlagMode, setIsFlagMode] = useState(false);
  const [flagCount, setFlagCount] = useState(0);
  const [isFirstRender, setIsFirstRender] = useState(true);
  const [lost, setLost] = useState(false);

  const getView = async () => await setView(await invoke("view"));
  const hasWon = async () => await invoke("won");
  const hasLost = async () => await invoke("lost");
  const resetViewAndFlagCount = async (viewNew) => {
    setFlagCount(0);
    setIsFlagMode(false);
    setView(viewNew);
  };
  const resetBoard = async () =>
    await resetViewAndFlagCount(await invoke("reset", { mineCount: 20 }));

  useEffect(() => {
    (async () => {
      if (isFirstRender == true) {
        getView();
        setIsFirstRender(false);
      }
      if ((await hasWon()) == true) {
        await resetBoard();
        await resetViewAndFlagCount();
        await getView();
        toast.success("You won ✨!", {
          position: "bottom-left",
          autoClose: 2000,
          hideProgressBar: false,
          closeOnClick: true,
          pauseOnHover: true,
          draggable: true,
          progress: undefined,
        });
      }
      await setLost(await hasLost());
    })();
  });

  return (
    <div className="h-screen bg-stone-800">
      <div className="flex justify-center items-center h-screen">
        <div className="w-80 h-94">
          <div className="grid border-4 border-gray-900">
            <GameMenu
              flagCount={flagCount}
              isFlagMode={isFlagMode}
              setIsFlagMode={setIsFlagMode}
              flagEmoji="🚩"
              otherEmoji={lost ? "😭" : "😃"}
              resetViewAndFlagCount={resetViewAndFlagCount}
            />
            <Board
              view={view ? view : []}
              setView={setView}
              isFlagMode={isFlagMode}
              setFlagCount={setFlagCount}
            />
          </div>
        </div>
      </div>
      <ToastContainer />
    </div>
  );
}

export default App;
