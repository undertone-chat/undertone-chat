import React, { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

type LogMessage = {
  message: string;
};
interface LogViewProps extends React.ComponentProps<"div"> {
  logs?: LogMessage[];
}

export const LogView = ({ ...props }: LogViewProps) => {
  const [logs, setLogs] = useState<LogMessage[]>([]);

  useEffect(() => {
    // 1. Define an async function to set up the listener.
    let unlisten;

    async function setupListener() {
      unlisten = await listen<LogMessage>("log-message", (event) => {
        console.log("Got poop!");
        setLogs([...logs, event.payload.message]);
      });
    }

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  return (
    <div {...props}>
      Im a log{" "}
      {logs &&
        logs.map((log, index) => {
          return (
            <div>
              <span key={index}>{log.message}</span>
            </div>
          );
        })}
    </div>
  );
};
