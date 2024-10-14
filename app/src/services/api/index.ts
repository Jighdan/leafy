import type { ModuleKeys } from "./interfaces";
import { ModuleDirectory } from "./modules";

type Modules = ModuleDirectory;

class Service {
	public module(_endpoint: ModuleKeys): Modules {
		return new ModuleDirectory();
	}
};

export const API = new Service();
