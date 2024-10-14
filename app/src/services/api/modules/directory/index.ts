import { CLIENT } from "../../client";
import type { Directory, DirectoryContent } from "./interfaces";

export class ModuleDirectory {
	private readonly client = CLIENT;
	private readonly endpoint = "/directory";

	public async get(): Promise<Directory[]>;
	public async get(path: string): Promise<DirectoryContent>;
	public async get(path?: string): Promise<Directory[] | DirectoryContent> {
		if (path) {
			const url = path.startsWith("/") ? `${this.endpoint}${path}` : `${this.endpoint}/${path}`;
			const { data } = await this.client.get<DirectoryContent>(url);
			
			return data;
		}

		const { data } = await this.client.get<Directory[]>(this.endpoint);

		return data;
	}
}