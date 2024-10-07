(async () => {
  const DATA_LIST = [
    "random",
    "stroll",
    "gaussian-with-noise",
    "low-sample-sin-with-noise",
    "high-sample-sin-with-noise",
    "trend-increasing",
  ];

  async function getData(dataName) {
    const DATA_DIRECTORY = "../bench-data/";
    var path = DATA_DIRECTORY + dataName + ".json";
    return await fetch(path).then((res) => res.json())
  }

  function selectGraphDom(dataName, parentDom = document.getElementById("graph")) {
    return parentDom.querySelector(`#graph-${dataName}`);
  }

  function displayGraph(dom, data, title = "") {
    // map title-snake-name to titleUpperCaseName
    title = title.split("-").map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(" ");

    var option = {
      title: {
        text: echarts.format.addCommas(title),
        left: 10
      },
      toolbox: {
        feature: {
          dataZoom: { yAxisIndex: false },
          saveAsImage: { pixelRatio: 2 }
        }
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'shadow' }
      },
      grid: { bottom: 90 },
      dataZoom: [
        { type: 'inside' },
        { type: 'slider' }
      ],
      xAxis: {
        data: [...Array(data.length).keys()],
        silent: false,
        splitLine: { show: false },
        splitArea: { show: false }
      },
      yAxis: {
        splitArea: { show: false },
        show: false
      },
      series: [{
        type: 'bar',
        data: data,
        large: true
      }]
    };

    var myChart = echarts.init(dom, null, {
      renderer: 'canvas',
      useDirtyRect: false
    });

    myChart.setOption(option);
    window.addEventListener('resize', myChart.resize);
  }

  for (var dataName of DATA_LIST) {
    var dom = selectGraphDom(dataName);
    var data = await getData(dataName);
    displayGraph(dom, data, dataName);
  }
})()
