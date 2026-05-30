<script setup lang="ts">
import Tags from './Tags.vue';

</script>

<template>
  <el-tabs tab-position="left" class="tabs" stretch>
    <el-tab-pane label="All">All</el-tab-pane>
    <el-tab-pane label="Tags"><Tags/></el-tab-pane>
    <el-tab-pane label="DataBase">DataBase</el-tab-pane>
    <el-tab-pane label="History">History</el-tab-pane>
    <el-tab-pane label="Analytics">Analytics</el-tab-pane>
    <el-tab-pane label="Configurations">Configurations</el-tab-pane>
  </el-tabs>
</template>

<style>
.el-tabs {
  height: 100%;
}

.el-tabs__header {
  background-color: #202227;
}

.el-tabs--left .el-tabs__content {
  height: 100%;
  width: 100%;
}

</style>