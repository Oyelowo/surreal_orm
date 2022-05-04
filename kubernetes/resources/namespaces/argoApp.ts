import { createArgocdChildrenApplication } from "../shared/createArgoApplication";
import { namespaceNames } from "./util";


export const namespacesArgoApps = createArgocdChildrenApplication({
    // resourceType: "namespaces",
    resourceName: "namespace-names",
    namespace: namespaceNames.default
});

