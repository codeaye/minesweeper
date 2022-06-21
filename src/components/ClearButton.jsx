import { invoke } from "@tauri-apps/api";
import React from "react";

const ClearButton = ({ resetViewAndFlagCount }) => {
  const resetBoard = async () =>
    await resetViewAndFlagCount(await invoke("reset"));

  return (
    <div
      onClick={() => resetBoard()}
      class="flex justify-center items-center grow h-14 text-2xl font-bold font-mono text-red-400 hover:text-red-300 bg-gray-700 hover:bg-gray-800 "
    >
      <div className="text-center">CLEAR</div>
    </div>
  );
};

export default ClearButton;
