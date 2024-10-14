import { API } from "~/services/api";

export default async function Page() {
  const directory = await API.module("directory").get();

  return (
    <div>
      <pre>{JSON.stringify(directory, null, 2)}</pre>
    </div>
  );
}
