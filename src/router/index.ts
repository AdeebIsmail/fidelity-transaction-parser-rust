import { createRouter, createWebHistory } from "vue-router";
import FileExplorer from "../views/FileExplorer.vue";
import CSVViewer from "../views/CSVViewer.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: FileExplorer,
    },
    {
      path: "/csv-viewer/:filename",
      name: "csv-viewer",
      component: CSVViewer,
      props: (route) => ({
        filename: route.params.filename,
        directoryPath: route.query.path || "",
      }),
    },
  ],
});

export default router;
