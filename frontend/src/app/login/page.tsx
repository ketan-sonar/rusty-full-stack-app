"use client";

import axios, { AxiosError } from "axios";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { FormEvent, useState } from "react";

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const router = useRouter();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      const res = await axios.post(
        "http://localhost:3001/login",
        {
          username,
          password,
        },
        { withCredentials: true },
      );
      alert(res.data.message);
      if (res.data.success) {
        console.log(document.cookie);
        router.push("/protected");
      }
    } catch (err: any) {
      if (err instanceof AxiosError) {
        alert(err.response?.data.message);
      } else {
        alert("Error logging in! Try again later...");
      }
    }
  };

  return (
    <main className="mx-auto flex min-h-screen max-w-md flex-col justify-center gap-4">
      <h1 className="text-center text-3xl font-bold">Login</h1>
      <form
        onSubmit={handleSubmit}
        className="flex flex-col justify-center gap-4"
      >
        <input
          type="text"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          className="rounded p-3 text-black"
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          className="rounded p-3 text-black"
        />
        <button className="rounded bg-blue-500 p-3 hover:bg-blue-500/90">
          Login
        </button>
      </form>
      <Link href="/register" className="text-center underline">
        Register Here
      </Link>
    </main>
  );
}
