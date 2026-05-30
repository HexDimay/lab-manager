<script setup>
import { Tag } from './Tag';

const tagsData = [
  new Tag("GLU", 19, "ss"),
];


</script>

<template>
  <el-table :data="tagsData" style="width: 100%">
    <el-table-column prop="name" label="Name tag" width="100">
      <template #default="scope">
        <el-tag>{{ scope.row.name }}</el-tag>
      </template>
    </el-table-column>
    <el-table-column prop="bind_count" label="Bind count" width="100" />
    <el-table-column prop="structure" label="Structure" />
  </el-table>
</template>