"use client";

import axios, { AxiosError } from "axios";
import { useRouter } from "next/navigation";
import { FormEvent, useState } from "react";

export default function Register() {
  const [name, setName] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const router = useRouter();

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      const res = await axios.post("http://localhost:3001/register", {
        username,
        name,
        password,
      });
      alert(res.data.message);
      if (res.data.success) {
        router.push("/login");
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
      <h1 className="text-center text-3xl font-bold">Register</h1>
      <form
        onSubmit={handleSubmit}
        className="flex flex-col justify-center gap-4"
      >
        <input
          type="text"
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="rounded p-3 text-black"
        />
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
          Register
        </button>
      </form>
    </main>
  );
}
