// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import { input as inputs, output as outputs } from "../../types";
import * as utilities from "../../utilities";

import {ObjectMeta} from "../../meta/v1";

/**
 * Prometheus defines a Prometheus deployment.
 */
export class Prometheus extends pulumi.CustomResource {
    /**
     * Get an existing Prometheus resource's state with the given name, ID, and optional extra
     * properties used to qualify the lookup.
     *
     * @param name The _unique_ name of the resulting resource.
     * @param id The _unique_ provider ID of the resource to lookup.
     * @param opts Optional settings to control the behavior of the CustomResource.
     */
    public static get(name: string, id: pulumi.Input<pulumi.ID>, opts?: pulumi.CustomResourceOptions): Prometheus {
        return new Prometheus(name, undefined as any, { ...opts, id: id });
    }

    /** @internal */
    public static readonly __pulumiType = 'kubernetes:monitoring.coreos.com/v1:Prometheus';

    /**
     * Returns true if the given object is an instance of Prometheus.  This is designed to work even
     * when multiple copies of the Pulumi SDK have been loaded into the same process.
     */
    public static isInstance(obj: any): obj is Prometheus {
        if (obj === undefined || obj === null) {
            return false;
        }
        return obj['__pulumiType'] === Prometheus.__pulumiType;
    }

    public readonly apiVersion!: pulumi.Output<"monitoring.coreos.com/v1" | undefined>;
    public readonly kind!: pulumi.Output<"Prometheus" | undefined>;
    public readonly metadata!: pulumi.Output<ObjectMeta | undefined>;
    /**
     * Specification of the desired behavior of the Prometheus cluster. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
     */
    public readonly spec!: pulumi.Output<outputs.monitoring.v1.PrometheusSpec>;
    /**
     * Most recent observed status of the Prometheus cluster. Read-only. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
     */
    public readonly status!: pulumi.Output<outputs.monitoring.v1.PrometheusStatus | undefined>;

    /**
     * Create a Prometheus resource with the given unique name, arguments, and options.
     *
     * @param name The _unique_ name of the resource.
     * @param args The arguments to use to populate this resource's properties.
     * @param opts A bag of options that control this resource's behavior.
     */
    constructor(name: string, args?: PrometheusArgs, opts?: pulumi.CustomResourceOptions) {
        let resourceInputs: pulumi.Inputs = {};
        opts = opts || {};
        if (!opts.id) {
            resourceInputs["apiVersion"] = "monitoring.coreos.com/v1";
            resourceInputs["kind"] = "Prometheus";
            resourceInputs["metadata"] = args ? args.metadata : undefined;
            resourceInputs["spec"] = args ? (args.spec ? pulumi.output(args.spec).apply(inputs.monitoring.v1.prometheusSpecArgsProvideDefaults) : undefined) : undefined;
            resourceInputs["status"] = args ? args.status : undefined;
        } else {
            resourceInputs["apiVersion"] = undefined /*out*/;
            resourceInputs["kind"] = undefined /*out*/;
            resourceInputs["metadata"] = undefined /*out*/;
            resourceInputs["spec"] = undefined /*out*/;
            resourceInputs["status"] = undefined /*out*/;
        }
        opts = pulumi.mergeOptions(utilities.resourceOptsDefaults(), opts);
        super(Prometheus.__pulumiType, name, resourceInputs, opts);
    }
}

/**
 * The set of arguments for constructing a Prometheus resource.
 */
export interface PrometheusArgs {
    apiVersion?: pulumi.Input<"monitoring.coreos.com/v1">;
    kind?: pulumi.Input<"Prometheus">;
    metadata?: pulumi.Input<ObjectMeta>;
    /**
     * Specification of the desired behavior of the Prometheus cluster. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
     */
    spec?: pulumi.Input<inputs.monitoring.v1.PrometheusSpecArgs>;
    /**
     * Most recent observed status of the Prometheus cluster. Read-only. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
     */
    status?: pulumi.Input<inputs.monitoring.v1.PrometheusStatusArgs>;
}
