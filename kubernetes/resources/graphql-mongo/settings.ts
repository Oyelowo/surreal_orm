import { devNamespaceName } from './../shared/namespaces';
import { Settings } from '../shared/types';

export const graphqlMongoSettings: Settings = {
  resourceName: "graphql-mongo",
  requestMemory: "70Mi",
  requestCpu: "100m",
  limitMemory: "200Mi",
  limitCpu: "100m",
  host: "0.0.0.0",
  image: "oyelowo/graphql-mongo",
};

type Environemt = "local" | "development" | "staging" | "production";

type AppEnvVars = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0" | string;
  APP_PORT: "8000" | `${number}`;
  MONGODB_NAME: string;
  MONGODB_USERNAME: string;
  MONGODB_PASSWORD: string;
  MONGODB_HOST: string;
  MONGODB_PORT: "27017";
  MONGODB_SERVICE_NAME: string;
};

 // `http://${name}.${namespace}:${port}`;

 /* 
 mongodb://username0@localhost:27017/?connectTimeoutMS=10000&authSource=db0&authMechanism=SCRAM-SHA-256&3t.uriVersion=3&3t.connection.name=db0&3t.alwaysShowAuthDB=true&3t.alwaysShowDBFromUserRole=true
 */

export const graphqlMongoEnvVars: AppEnvVars = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  MONGODB_NAME: "db0",
  MONGODB_USERNAME: "username0",
  MONGODB_PASSWORD: "password0",
  MONGODB_HOST: `mongo-database.${devNamespaceName}`,
  MONGODB_SERVICE_NAME: "mongo-database",
  // hostAndPort":"graphql-mongo-0.mongo-graphql.development.svc.cluster.local:27017
  // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
  // const url = 'mongodb://username1:$[password]@mongo-graphql.development:27017/db1?authSource=$[authSource]';

  MONGODB_PORT: "27017",
} as const;