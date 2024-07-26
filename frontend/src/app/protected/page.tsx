"use client";

import axios, { AxiosError } from "axios";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Protected() {
  const [username, setUsername] = useState("");

  const router = useRouter();

  useEffect(() => {
    async function sendApiRequest() {
      try {
        const res = await axios.get("http://localhost:3001/protected", {
          withCredentials: true,
        });
        alert(res.data.message);
        if (res.data.success) {
          setUsername(res.data.username);
        } else {
          router.push("/login");
        }
      } catch (err: any) {
        if (err instanceof AxiosError) {
          alert(err.response?.data.message);
        } else {
          alert("Could not fetch data! Please try again later...");
        }
        router.push("/login");
      }
    }

    sendApiRequest();
  }, [router]);

  return (
    <main className="mx-auto flex min-h-screen max-w-md flex-col items-center justify-center gap-4">
      <h1 className="text-4xl font-bold">Protected Page</h1>
      <p className="text-xl">Hello {username}!</p>
    </main>
  );
}
