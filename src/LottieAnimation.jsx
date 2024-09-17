// LottieAnimation.jsx
import React, { useEffect, useRef } from 'react';
import lottie from 'lottie-web';

const LottieAnimation = () => {
  const containerRef = useRef(null);

  useEffect(() => {
    if (containerRef.current) {
      lottie.loadAnimation({
        container: containerRef.current,
        renderer: 'svg',
        loop: true,
        autoplay: true,
        path: '/data.json', // 替换为你的动画文件路径
      });
    }
  }, []);

  return <div ref={containerRef}></div>;
};

export default LottieAnimation;
