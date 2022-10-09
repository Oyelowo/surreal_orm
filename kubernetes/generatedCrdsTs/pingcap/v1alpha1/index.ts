// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import * as utilities from "../../utilities";

// Export members:
export * from "./backup";
export * from "./backupSchedule";
export * from "./dmcluster";
export * from "./restore";
export * from "./tidbCluster";
export * from "./tidbClusterAutoScaler";
export * from "./tidbInitializer";
export * from "./tidbMonitor";
export * from "./tidbNGMonitoring";

// Import resources to register:
import { Backup } from "./backup";
import { BackupSchedule } from "./backupSchedule";
import { DMCluster } from "./dmcluster";
import { Restore } from "./restore";
import { TidbCluster } from "./tidbCluster";
import { TidbClusterAutoScaler } from "./tidbClusterAutoScaler";
import { TidbInitializer } from "./tidbInitializer";
import { TidbMonitor } from "./tidbMonitor";
import { TidbNGMonitoring } from "./tidbNGMonitoring";

const _module = {
    version: utilities.getVersion(),
    construct: (name: string, type: string, urn: string): pulumi.Resource => {
        switch (type) {
            case "kubernetes:pingcap.com/v1alpha1:Backup":
                return new Backup(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:BackupSchedule":
                return new BackupSchedule(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:DMCluster":
                return new DMCluster(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:Restore":
                return new Restore(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:TidbCluster":
                return new TidbCluster(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:TidbClusterAutoScaler":
                return new TidbClusterAutoScaler(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:TidbInitializer":
                return new TidbInitializer(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:TidbMonitor":
                return new TidbMonitor(name, <any>undefined, { urn })
            case "kubernetes:pingcap.com/v1alpha1:TidbNGMonitoring":
                return new TidbNGMonitoring(name, <any>undefined, { urn })
            default:
                throw new Error(`unknown resource type ${type}`);
        }
    },
};
pulumi.runtime.registerResourceModule("crds", "pingcap.com/v1alpha1", _module)
